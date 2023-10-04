use itertools::Itertools;

use crate::*;

impl Shape {
    pub const WIDTH: usize = 3;
    pub const HEIGHT: usize = 3;

    pub fn from_binary(arr: [u8; 3]) -> Self {
        let bits_rev = (arr[2] as u16) | ((arr[1] as u16) << 3) | ((arr[0] as u16) << 6);
        let bits = bits_rev.reverse_bits() >> (u16::BITS as usize - Self::WIDTH * Self::HEIGHT);

        Self(bits)
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        debug_assert!(x < Self::WIDTH);
        debug_assert!(y < Self::HEIGHT);

        let index = y * Self::WIDTH + x;
        self.0 & (1 << index) != 0
    }

    // TODO: interesting optimization candidate after we remove heap allocs
    // - we can calculate the maximum number of tiles that can be covered by a shape
    // - it is likely faster to query individual tiles rather than get a list
    pub fn get_neighbours(&self) -> ShapeNeighbours {
        let width = Self::WIDTH as isize;
        let height = Self::HEIGHT as isize;

        let mut neighbours = ShapeNeighbours::default();
        'a: for (probe_y, probe_x) in (-1..=height).cartesian_product(-1..=width) {
            if (0..width).contains(&probe_x) && (0..height).contains(&probe_y) {
                // coordinate is inside the shape boundaries
                // if it's part of the shape, it's not a neighbour
                if self.get(probe_x as usize, probe_y as usize) {
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
                    if self.get(neighbour.0 as usize, neighbour.1 as usize) {
                        neighbours.set(probe_x, probe_y);
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
            if self.get(x, y) {
                max_x = max_x.max(x);
            }
        }
        max_x
    }

    pub fn get_max_y(&self) -> usize {
        let mut max_y = 0;
        for (y, x) in (0..Self::HEIGHT).cartesian_product(0..Self::WIDTH) {
            if self.get(x, y) {
                max_y = max_y.max(y);
            }
        }
        max_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_binary() {
        assert_eq!(0, Shape::from_binary([0b000, 0b000, 0b000]).0);
        assert_eq!(0b111_111_111, Shape::from_binary([0b111, 0b111, 0b111]).0);

        assert_eq!(0b011_101_110, Shape::from_binary([0b011, 0b101, 0b110]).0);
    }

    #[test]
    fn test_get() {
        let shape = Shape::from_binary([0b010, 0b101, 0b110]);

        assert!(!shape.get(0, 0));
        assert!(shape.get(1, 0));
        assert!(!shape.get(2, 0));
        assert!(shape.get(0, 1));
        assert!(!shape.get(1, 1));
        assert!(shape.get(2, 1));
        assert!(shape.get(0, 2));
        assert!(shape.get(1, 2));
        assert!(!shape.get(2, 2));
    }
}
