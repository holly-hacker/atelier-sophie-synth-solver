use itertools::Itertools;

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
        let (row, col) = index;
        let index = row * self.size + col;
        *self.tiles.get(index).unwrap()
    }

    pub fn get_tile_mut(&mut self, index: (usize, usize)) -> &mut Option<Tile> {
        debug_assert!(index.0 < self.size);
        debug_assert!(index.1 < self.size);

        let (x, y) = index;
        let index = y * self.size + x;

        self.tiles.get_mut(index).unwrap()
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
        item_index: (usize, usize),
        placement: Placement,
        allow_overlap: bool,
        scores: &mut [ColorScoreSet],
    ) -> Result<(), SynthError> {
        debug_assert_eq!(material_groups.len(), scores.len());

        let material = material_groups[item_index.0][item_index.1];
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
                let tile = self
                    .get_tile_mut((placement_x + shape_x, placement_y + shape_y))
                    .as_mut()
                    .expect("cannot place item on unavailable tile");
                if tile.played_color.is_some() {
                    if allow_overlap {
                        todo!("implement tile overlap");
                    } else {
                        return Err(SynthError::DisallowedOverlap);
                    }
                }
                tile.played_color = Some(material.color);

                // TODO: this currently assumes a "Grandma's Cauldron"
                // this means that matching colors give 50% bonus (rounded down) and tiles give 3/5/7 points
                let bonus = if material.color == tile.color {
                    1.5
                } else {
                    1.
                };
                score += match tile.level {
                    0 => 0.,
                    1 => 3. * bonus,
                    2 => 5. * bonus,
                    3 => 7. * bonus,
                    n => unreachable!("invalid tile level: {n}"),
                };
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
                if tile.played_color.is_some() {
                    continue;
                }

                // tiles can only go up to level 3
                if tile.level < 3 {
                    tile.level += 1;
                }
            }
        }

        // apply score now, after possible errors
        *scores[item_index.0].get_mut(material.color) += score;

        Ok(())
    }

    pub fn calculate_coverage(&self) -> CoverageInfo {
        self.tiles
            .iter()
            .filter_map(|t| t.as_ref())
            .filter_map(|t| t.played_color)
            .fold(CoverageInfo::default(), CoverageInfo::add_color)
    }
}
