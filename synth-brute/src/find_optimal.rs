use crate::*;

#[derive(Debug, Clone)]
pub struct Move {
    pub material_index: (usize, usize),
    pub placement: Placement,
}

// TODO: pass goals as well
// TODO: pass configuration to allow overlaps/transformations
pub fn find_optimal(playfield: &Cauldron, materials: &[Vec<Material>]) -> Option<Vec<Move>> {
    let path: Vec<Move> = vec![];
    let scores = vec![ColorScoreSet::default(); materials.len()];
    let mut max_score = 0;
    find_optimal_recursive(playfield, materials, path, scores, &mut max_score)
}

fn find_optimal_recursive(
    playfield: &Cauldron,
    materials: &[Vec<Material>],
    path: Vec<Move>,
    scores: Vec<ColorScoreSet>,
    max_score: &mut u32,
) -> Option<Vec<Move>> {
    if path.len() == materials.iter().map(|m| m.len()).sum::<usize>() {
        let coverage = playfield.calculate_coverage();
        let score = scores
            .iter()
            .enumerate()
            .map(|(i, s)| s.calculate_score(&materials[i], &coverage, playfield))
            .sum::<u32>();
        if score > *max_score {
            *max_score = score;
            return Some(path.clone());
        } else {
            return None;
        }
    }

    let mut best_path: Option<Vec<Move>> = None;
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
                let mut new_scores = scores.clone();
                if new_playfield
                    .place(
                        materials,
                        (material_group_index, material_index),
                        placement,
                        &mut new_scores,
                    )
                    .is_ok()
                {
                    if let Some(path) = find_optimal_recursive(
                        &new_playfield,
                        materials,
                        new_path,
                        new_scores,
                        max_score,
                    ) {
                        best_path = Some(path);
                    }
                }
            }
        }
    }

    best_path
}
