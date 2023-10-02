use itertools::Itertools;

use crate::*;

impl Shape {
    pub const WIDTH: usize = 3;
    pub const HEIGHT: usize = 3;

    // TODO: interesting optimization candidate after we remove heap allocs
    // - we can calculate the maximum number of tiles that can be covered by a shape
    // - it is likely faster to query individual tiles rather than get a list
    pub fn get_neighbouring_tiles(&self) -> Vec<(isize, isize)> {
        let width = Self::WIDTH as isize;
        let height = Self::HEIGHT as isize;

        let mut neighbours = Vec::new();
        'a: for (probe_y, probe_x) in (-1..=height).cartesian_product(-1..=width) {
            if (0..width).contains(&probe_x) && (0..height).contains(&probe_y) {
                // coordinate is inside the shape boundaries
                // if it's part of the shape, it's not a neighbour
                if self.0[probe_y as usize][probe_x as usize] {
                    continue;
                }
            }

            // check neighbours
            for (dy, dx) in (-1..=1).cartesian_product(-1..=1) {
                if (dy, dx) == (0, 0) {
                    continue;
                }

                let neighbour = (probe_x + dx, probe_y + dy);
                if (0..width).contains(&neighbour.0) && (0..height).contains(&neighbour.1) {
                    // coordinate is inside the shape boundaries
                    // if it's part of the shape, it's not a neighbour
                    if self.0[neighbour.1 as usize][neighbour.0 as usize] {
                        neighbours.push((probe_x, probe_y));
                        continue 'a;
                    }
                }
            }
        }
        neighbours
    }

    pub fn get_max_x(&self) -> usize {
        let mut max_x = 0;
        for (y, x) in (0..Self::HEIGHT).cartesian_product(0..Self::WIDTH) {
            if self.0[y][x] {
                max_x = max_x.max(x);
            }
        }
        max_x
    }

    pub fn get_max_y(&self) -> usize {
        let mut max_y = 0;
        for (y, x) in (0..Self::HEIGHT).cartesian_product(0..Self::WIDTH) {
            if self.0[y][x] {
                max_y = max_y.max(y);
            }
        }
        max_y
    }
}
