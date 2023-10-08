mod goal_result;
mod progress_tracker;

use std::ops::ControlFlow;

use tinyvec::ArrayVec;

use crate::*;
pub use goal_result::*;
pub use progress_tracker::ProgressReporter;
use progress_tracker::ProgressTracker;

pub type SolverResult = Vec<(GoalResult, ArrayVec<[Move; MAX_ITEMS]>)>;

#[derive(Default, Clone)]
pub struct SolverSettings {
    /// The allowed transformations
    pub transformations: TransformationType,
    /// Whether to allow overlapping placements
    pub allow_overlaps: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Move {
    pub material_index: (usize, usize),
    pub placement: Placement,
}

pub fn find_optimal_routes(
    playfield: &Cauldron,
    materials: &[Vec<Material>],
    goals: &[Goal],
    properties: &SolverSettings,
    progress_reporter: Option<ProgressReporter>,
) -> SolverResult {
    assert_eq!(materials.len(), goals.len());

    Shape::init_neighbour_cache();

    let path: ArrayVec<[Move; MAX_ITEMS]> = Default::default();
    let mut score_sets: ArrayVec<[ColorScoreSet; MAX_GOALS]> = Default::default();
    for _ in 0..materials.len() {
        score_sets.push(Default::default());
    }

    let mut progress_tracker = ProgressTracker::new(progress_reporter);

    let mut max_scores = Default::default();

    find_optimal_recursive(
        playfield,
        materials,
        goals,
        properties,
        &mut progress_tracker,
        path,
        score_sets,
        &mut max_scores,
    );

    max_scores
}

#[allow(clippy::too_many_arguments)] // FIXME
fn find_optimal_recursive(
    playfield: &Cauldron,
    materials: &[Vec<Material>],
    goals: &[Goal],
    properties: &SolverSettings,
    progress_tracker: &mut ProgressTracker,
    path: ArrayVec<[Move; MAX_ITEMS]>,
    score_sets: ArrayVec<[ColorScoreSet; MAX_GOALS]>,
    max_scores: &mut SolverResult,
) -> ControlFlow<()> {
    let current_depth = path.len();
    let material_count = materials.iter().map(|m| m.len()).sum::<usize>();
    let desired_depth = material_count;

    if current_depth == desired_depth {
        return check_end_of_path(
            playfield,
            materials,
            goals,
            progress_tracker,
            path,
            score_sets,
            max_scores,
        );
    }

    let material_count_in_current_iteration = material_count - current_depth;
    progress_tracker.start_loop(material_count_in_current_iteration);
    for (material_group_index, material_group) in materials.iter().enumerate() {
        for (material_index, _) in
            material_group
                .iter()
                .enumerate()
                .filter(|(material_index, _)| {
                    !path
                        .iter()
                        .any(|m| m.material_index == (material_group_index, *material_index))
                })
        {
            progress_tracker.report_progress()?;

            let transformations = generate_transformations(
                materials[material_group_index][material_index].shape,
                properties.transformations,
            );
            progress_tracker.start_loop(transformations.len());
            for transformation in transformations {
                progress_tracker.report_progress()?;

                progress_tracker.start_loop(playfield.tiles.len());
                for playfield_index in 0..playfield.tiles.len() {
                    progress_tracker.report_progress()?;

                    let placement = Placement::new(playfield_index, transformation);
                    let mut new_path = path.clone();
                    new_path.push(Move {
                        material_index: (material_group_index, material_index),
                        placement,
                    });
                    let mut new_playfield = playfield.clone();
                    let mut new_score_sets = score_sets;
                    if new_playfield
                        .place(
                            materials,
                            (material_group_index, material_index),
                            placement,
                            properties.allow_overlaps,
                            &mut new_score_sets,
                        )
                        .is_ok()
                    {
                        find_optimal_recursive(
                            &new_playfield,
                            materials,
                            goals,
                            properties,
                            progress_tracker,
                            new_path,
                            new_score_sets,
                            max_scores,
                        )?;
                    }
                    progress_tracker.bump_loop_progress(); // playfield tiles
                }
                progress_tracker.end_loop(); // playfield tiles
                progress_tracker.bump_loop_progress(); // transformations
            }
            progress_tracker.end_loop(); // transformations
            progress_tracker.bump_loop_progress(); // materials
        }
    }
    progress_tracker.end_loop(); // materials

    ControlFlow::Continue(())
}

fn check_end_of_path(
    playfield: &Cauldron,
    materials: &[Vec<Material>],
    goals: &[Goal],
    _progress_tracker: &mut ProgressTracker,
    path: ArrayVec<[Move; MAX_ITEMS]>,
    score_sets: ArrayVec<[ColorScoreSet; MAX_GOALS]>,
    max_scores: &mut SolverResult,
) -> ControlFlow<()> {
    let coverage = playfield.calculate_coverage();
    let scores = score_sets
        .iter()
        .enumerate()
        .map(|(i, s)| s.calculate_score(&materials[i], &coverage, playfield))
        .collect::<ArrayVec<[_; MAX_GOALS]>>();
    let current_results = GoalResult::from_scores(&scores, goals);

    max_scores.retain(|r| !current_results.is_strictly_better(&r.0));

    if max_scores.is_empty()
        || (!max_scores.iter().any(|ms| ms.0 == current_results)
            && max_scores
                .iter()
                .all(|ms| !ms.0.is_strictly_better(&current_results)))
    {
        max_scores.push((current_results.clone(), path));

        // check if we reached a perfect score, which is where we meet all goals
        if current_results
            .achieved_goals
            .iter()
            .zip(goals.iter().map(|g| g.effect_value_thresholds.len()))
            .all(|(s, g)| *s == g)
        {
            return ControlFlow::Break(());
        }
    }

    ControlFlow::Continue(())
}

// at most, this should return 4 permutations (for rotation)
fn generate_transformations(
    shape: Shape,
    transformation_type: TransformationType,
) -> ArrayVec<[Option<Transformation>; 4]> {
    let mut ret = ArrayVec::new();
    ret.push(None);

    // we apply the transformation first to see if there's an actual change, to prevent doing duplicate work
    // PERF: this can probably be micro-optimized to avoid having to apply the actual transformation
    match transformation_type {
        TransformationType::None => {}
        TransformationType::FlipHorizontal => {
            if shape.apply_transformation(Transformation::FlipHorizontal) != shape {
                ret.push(Some(Transformation::FlipHorizontal));
            }
        }
        TransformationType::FlipVertical => {
            if shape.apply_transformation(Transformation::FlipVertical) != shape {
                ret.push(Some(Transformation::FlipVertical));
            }
        }
        TransformationType::Rotate => {
            if shape.apply_transformation(Transformation::Rotate90) != shape {
                ret.push(Some(Transformation::Rotate90));

                if shape.apply_transformation(Transformation::Rotate90)
                    != shape.apply_transformation(Transformation::Rotate270)
                {
                    ret.push(Some(Transformation::Rotate270));
                }
            }
            if shape.apply_transformation(Transformation::Rotate180) != shape {
                ret.push(Some(Transformation::Rotate180));
            }
        }
    };

    ret
}
