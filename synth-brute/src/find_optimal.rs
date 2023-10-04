use crate::*;

use tinyvec::ArrayVec;

#[derive(Debug, Clone, Default)]
pub struct Move {
    pub material_index: (usize, usize),
    pub placement: Placement,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GoalResult {
    /// The amount of thresholds that are met for each goal.
    pub scores: ArrayVec<[usize; MAX_GOALS]>,
}

impl GoalResult {
    pub fn from_scores(scores: &[u32], goals: &[Goal]) -> Self {
        debug_assert_eq!(scores.len(), goals.len());
        Self {
            scores: scores
                .iter()
                .zip(goals.iter())
                .map(|(s, g)| g.effect_value_thresholds.iter().filter(|t| s >= t).count())
                .collect(),
        }
    }

    pub fn is_strictly_better(&self, other: &GoalResult) -> bool {
        debug_assert_eq!(self.scores.len(), other.scores.len());
        self.scores
            .iter()
            .zip(other.scores.iter())
            .all(|(a, b)| a >= b)
    }
}

// TODO: pass configuration to allow overlaps/transformations
pub fn find_optimal_routes(
    playfield: &Cauldron,
    materials: &[Vec<Material>],
    goals: &[Goal],
) -> Vec<(GoalResult, ArrayVec<[Move; MAX_ITEMS]>)> {
    assert_eq!(materials.len(), goals.len());

    Shape::init_neighbour_cache();

    let path: ArrayVec<[Move; MAX_ITEMS]> = Default::default();
    let mut score_sets: ArrayVec<[ColorScoreSet; MAX_GOALS]> = Default::default();
    for _ in 0..materials.len() {
        score_sets.push(Default::default());
    }

    let mut max_scores = Default::default();
    find_optimal_recursive(
        playfield,
        materials,
        goals,
        path,
        score_sets,
        &mut max_scores,
    );

    max_scores
}

fn find_optimal_recursive(
    playfield: &Cauldron,
    materials: &[Vec<Material>],
    goals: &[Goal],
    path: ArrayVec<[Move; MAX_ITEMS]>,
    score_sets: ArrayVec<[ColorScoreSet; MAX_GOALS]>,
    max_scores: &mut Vec<(GoalResult, ArrayVec<[Move; MAX_ITEMS]>)>,
) {
    if path.len() == materials.iter().map(|m| m.len()).sum::<usize>() {
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
            max_scores.push((current_results, path.clone()));
        }
    }

    for (material_group_index, material_group) in materials.iter().enumerate() {
        for (material_index, _) in material_group.iter().enumerate() {
            // we can't re-use materials
            if path
                .iter()
                .any(|m| m.material_index == (material_group_index, material_index))
            {
                continue;
            }

            // TODO: also iterate over possible transformations of the tile (make sure to dedupe too)
            for playfield_index in 0..playfield.tiles.len() {
                let placement = Placement::new(playfield_index, ());
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
                        &mut new_score_sets,
                    )
                    .is_ok()
                {
                    find_optimal_recursive(
                        &new_playfield,
                        materials,
                        goals,
                        new_path,
                        new_score_sets,
                        max_scores,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinyvec::array_vec;

    macro_rules! is_strictly_better {
        (better: $(($a:expr, $b:expr),)*) => {
            $(
                assert!(GoalResult { scores: $a }.is_strictly_better(&GoalResult { scores: $b }));
            )*
        };
        (not better: $(($a:expr, $b:expr),)*) => {
            $(
                assert!(!GoalResult { scores: $a }.is_strictly_better(&GoalResult { scores: $b }));
            )*
        };
    }

    #[test]
    fn test_strictly_better() {
        is_strictly_better![
            better:
            (array_vec![1, 1, 1], array_vec![1, 0, 1]),
            (array_vec![1, 1, 1], array_vec![0, 1, 1]),
            (array_vec![1, 1, 1], array_vec![0, 0, 1]),
            (array_vec![1, 0, 0], array_vec![0, 0, 0]),
        ];
        is_strictly_better![
            not better:
            (array_vec![1, 1, 1], array_vec![2, 0, 0]),
            (array_vec![1, 0, 1], array_vec![1, 1, 0]),
            (array_vec![0, 0, 1], array_vec![2, 0, 0]),
        ];
    }
}
