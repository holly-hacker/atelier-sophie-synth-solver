use std::sync::OnceLock;

use itertools::Itertools;

use crate::*;

/// A shape is a 3x3 grid of tiles that can be placed on the playfield, so there are 2^(3*3)=2^9=512
/// possible shapes. We can cache all of them fairly easily since we can store the neighbours in a
/// single u32, so this entire cache takes up 2kb.
static NEIGHBOUR_CACHE: OnceLock<[ShapeNeighbours; 512]> = OnceLock::new();

impl Shape {
    pub const WIDTH: usize = 3;
    pub const HEIGHT: usize = 3;

    /// Initialize the neighbour cache so it doesn't impact performance measurements in a weird way.
    pub fn init_neighbour_cache() {
        _ = Self(0).get_neighbours();
    }

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

    pub fn get_neighbours(&self) -> ShapeNeighbours {
        NEIGHBOUR_CACHE.get_or_init(|| {
            let mut cache = [ShapeNeighbours::default(); 512];
            for (i, cache_item) in cache.iter_mut().enumerate() {
                *cache_item = Self(i as u16).calculate_neighbours();
            }
            cache
        })[self.0 as usize]
    }

    fn calculate_neighbours(&self) -> ShapeNeighbours {
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
        debug_assert_ne!(self.0, 0);
        debug_assert_eq!(Self::WIDTH, 3);
        debug_assert_eq!(Self::HEIGHT, 3);

        if self.0 & 0b100_100_100 != 0 {
            2
        } else if self.0 & 0b010_010_010 != 0 {
            1
        } else {
            0
        }
    }

    pub fn get_max_y(&self) -> usize {
        debug_assert_ne!(self.0, 0);
        debug_assert_eq!(Self::WIDTH, 3);
        debug_assert_eq!(Self::HEIGHT, 3);

        if self.0 & 0b111_000_000 != 0 {
            2
        } else if self.0 & 0b000_111_000 != 0 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_binary() {
        assert_eq!(0, Shape::from_binary([0b000, 0b000, 0b000]).0);
        assert_eq!(0b111_111_111, Shape::from_binary([0b111, 0b111, 0b111]).0);
        assert_eq!(0b011_101_010, Shape::from_binary([0b010, 0b101, 0b110]).0);
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

    #[test]
    fn test_cache() {
        Shape::init_neighbour_cache();

        for i in 0..512 {
            let shape = Shape(i as u16);
            assert_eq!(shape.get_neighbours(), shape.calculate_neighbours());
        }

        let shape = Shape::from_binary([0b010, 0b101, 0b110]);
        assert_eq!(shape.get_neighbours(), shape.calculate_neighbours());
    }
}
