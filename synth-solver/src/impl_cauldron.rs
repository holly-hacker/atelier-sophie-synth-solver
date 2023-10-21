use itertools::Itertools;
use tinyvec::ArrayVec;

use crate::solver::Move;
use crate::{errors::SynthError, *};

impl Cauldron {
    pub fn get_position(&self, index: usize) -> (usize, usize) {
        debug_assert!(index < self.tiles.len());
        (index % self.size, index / self.size)
    }

    pub fn get_tile(&self, index: (usize, usize)) -> Option<Tile> {
        debug_assert!(index.0 < self.size);
        debug_assert!(index.1 < self.size);

        let (x, y) = index;
        let index = y * self.size + x;

        *self.tiles.get(index).unwrap()
    }

    pub fn get_tile_mut(&mut self, index: (usize, usize)) -> &mut Option<Tile> {
        debug_assert!(index.0 < self.size);
        debug_assert!(index.1 < self.size);

        let (x, y) = index;
        let index = y * self.size + x;

        self.tiles.get_mut(index).unwrap()
    }

    pub fn calculate_final_score(
        &self,
        material_groups: &[Vec<Material>],
        score_sets: &[ColorScoreSet],
    ) -> ArrayVec<[u32; MAX_GOALS]> {
        let coverage = self.calculate_coverage(material_groups);

        let scores = score_sets
            .iter()
            .enumerate()
            .map(|(i, s)| s.calculate_score(&material_groups[i], &coverage, self))
            .collect::<ArrayVec<[_; MAX_GOALS]>>();

        scores
    }

    pub fn place_all(
        &mut self,
        material_groups: &[Vec<Material>],
        moves: &[Move],
        allow_overlap: bool,
    ) -> Result<Vec<ColorScoreSet>, SynthError> {
        let mut scores = vec![ColorScoreSet::default(); material_groups.len()];
        for move_ in moves {
            self.place(
                material_groups,
                move_.material_index,
                move_.placement,
                allow_overlap,
                &mut scores,
            )?;
        }

        Ok(scores)
    }

    pub fn place(
        &mut self,
        material_groups: &[Vec<Material>],
        material_index: (usize, usize),
        placement: Placement,
        allow_overlap: bool,
        scores: &mut [ColorScoreSet],
    ) -> Result<(), SynthError> {
        debug_assert_eq!(material_groups.len(), scores.len());

        let material = material_groups[material_index.0][material_index.1];
        let shape = match placement.transformation {
            Some(transformation) => material.shape.apply_transformation(transformation),
            None => material.shape.normalize(),
        };
        let (placement_x, placement_y) = self.get_position(placement.index);

        if placement_x + shape.get_max_x() >= self.size
            || placement_y + shape.get_max_y() >= self.size
        {
            return Err(SynthError::OutOfBounds);
        }

        // apply the shape to the playfield and count score
        let mut score = 0.;
        for (shape_y, shape_x) in (0..Shape::HEIGHT).cartesian_product(0..Shape::WIDTH) {
            if shape.get(shape_x, shape_y) {
                // copy value to avoid borrow checker issues
                let bonus_scores = self.bonus_scores;
                let properties = self.properties;

                let tile = self
                    .get_tile_mut((placement_x + shape_x, placement_y + shape_y))
                    .as_mut()
                    .expect("cannot place item on unavailable tile");

                if tile.played_material_index.is_some() && !allow_overlap {
                    return Err(SynthError::DisallowedOverlap);
                }

                // matching colors give a 50% bonus to bonus score
                let bonus_multiplier = if material.color == tile.color {
                    1.5
                } else {
                    1.
                };
                score += match tile.level {
                    0 => 0.,
                    1 => bonus_scores.0 as f32 * bonus_multiplier,
                    2 => bonus_scores.1 as f32 * bonus_multiplier,
                    3 => bonus_scores.2 as f32 * bonus_multiplier,
                    n => unreachable!("invalid tile level: {n}"),
                };

                // TODO: handle synergy bonus
                if properties.contains(CauldronProperties::SYNERGY) {
                    todo!("synergy bonus not implemented");
                }

                let material_index_before_placement = tile.played_material_index;

                tile.played_material_index = Some(material_index);
                tile.level = 0;

                if let Some(material_index) = material_index_before_placement {
                    debug_assert!(allow_overlap); // checked earlier

                    // clear all tiles that have the old material index
                    for other_tile in self.tiles.iter_mut().filter_map(|t| t.as_mut()) {
                        if other_tile.played_material_index == Some(material_index) {
                            other_tile.played_material_index = None;
                        }
                    }
                }
            }
        }
        // score is truncated into an integer
        let score = score as u32;

        // increment the neighbours of this shape
        let neighbours = shape.get_neighbours();

        for (neighbour_x, neighbour_y) in neighbours {
            let position_x = placement_x as isize + neighbour_x;
            let position_y = placement_y as isize + neighbour_y;

            if !(0..self.size as isize).contains(&position_x)
                || !(0..self.size as isize).contains(&position_y)
            {
                continue;
            }

            let tile = self.get_tile_mut((position_x as usize, position_y as usize));

            if let Some(tile) = tile {
                // tiles that are already played cannot be updated
                if tile.played_material_index.is_some() {
                    continue;
                }

                // tiles can only go up to level 3
                if tile.level < 3 {
                    tile.level += 1;
                }
            }
        }

        // apply score now, after possible errors
        *scores[material_index.0].get_mut(material.color) += score;

        Ok(())
    }

    pub fn calculate_coverage(&self, material_groups: &[Vec<Material>]) -> CoverageInfo {
        self.tiles
            .iter()
            .filter_map(|t| t.as_ref())
            .filter_map(|t| t.played_material_index)
            .map(|(i1, i2)| material_groups[i1][i2].color)
            .fold(CoverageInfo::default(), CoverageInfo::add_color)
    }
}
