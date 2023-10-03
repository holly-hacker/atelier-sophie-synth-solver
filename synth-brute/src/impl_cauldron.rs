use crate::*;

use itertools::Itertools;

impl Cauldron {
    pub fn height(&self) -> usize {
        self.tiles.len() / self.size
    }

    pub fn get_position(&self, index: usize) -> (usize, usize) {
        debug_assert!(index < self.tiles.len());
        (index % self.size, index / self.size)
    }

    pub fn get_tile(&self, index: (usize, usize)) -> Option<Tile> {
        debug_assert!(index.0 < self.size);
        debug_assert!(index.0 < self.height());
        let (row, col) = index;
        let index = row * self.size + col;
        *self.tiles.get(index).unwrap()
    }

    pub fn get_tile_mut(&mut self, index: (usize, usize)) -> Option<&mut Tile> {
        debug_assert!(index.0 < self.size);
        debug_assert!(index.0 < self.height());

        let (x, y) = index;
        let index = y * self.size + x;

        self.tiles.get_mut(index).unwrap().as_mut()
    }

    pub fn place(
        &mut self,
        material_groups: &[Vec<Material>],
        item_index: (usize, usize),
        placement: Placement,
    ) -> usize {
        let material = material_groups[item_index.0][item_index.1];
        let shape = &material.shape;
        let (placement_x, placement_y) = self.get_position(placement.index);
        debug_assert!(placement_x + shape.get_max_x() <= self.size);
        debug_assert!(placement_y + shape.get_max_y() <= self.height());

        // apply the shape to the playfield and count score
        let mut score = 0.;
        for (shape_y, shape_x) in (0..Shape::HEIGHT).cartesian_product(0..Shape::WIDTH) {
            if shape.0[shape_y][shape_x] {
                let tile = self
                    .get_tile_mut((placement_x + shape_x, placement_y + shape_y))
                    .expect("cannot place item on unavailable tile");
                assert!(
                    tile.played_color.is_none(),
                    "overlapping tiles is not yet implemented"
                );
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
        let score = score as usize;

        // increment the neighbours of this shape
        let neighbour_offsets = shape.get_neighbouring_tiles();

        for (neighbour_x, neighbour_y) in neighbour_offsets {
            let position_x = placement_x as isize + neighbour_x;
            let position_y = placement_y as isize + neighbour_y;

            if !(0..self.size as isize).contains(&position_x)
                || !(0..self.height() as isize).contains(&position_y)
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

        score
    }

    pub fn calculate_coverage(&self) -> CoverageInfo {
        self.tiles
            .iter()
            .filter_map(|t| t.as_ref())
            .filter_map(|t| t.played_color)
            .fold(CoverageInfo::default(), |acc, color| acc.add_color(color))
    }
}
