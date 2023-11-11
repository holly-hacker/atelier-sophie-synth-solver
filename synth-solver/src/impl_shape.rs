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
    pub const MAX_SHAPE_COUNT: u16 = 2u16.pow((Self::WIDTH * Self::HEIGHT) as u32);

    /// Initialize the neighbour cache so it doesn't impact performance measurements in a weird way.
    pub fn init_neighbour_cache() {
        _ = Self(0).get_neighbours();
    }

    pub fn from_indices(iter: impl Iterator<Item = usize>) -> Self {
        let mut bits = 0;
        for index in iter {
            bits |= 1 << index;
        }
        Self(bits)
    }

    pub fn from_binary(arr: [u8; 3]) -> Self {
        debug_assert_eq!(Self::WIDTH, 3);
        debug_assert_eq!(Self::HEIGHT, 3);

        let bits_rev = (arr[2] as u16) | ((arr[1] as u16) << 3) | ((arr[0] as u16) << 6);
        let bits = bits_rev.reverse_bits() >> (u16::BITS as usize - Self::WIDTH * Self::HEIGHT);

        Self(bits)
    }

    pub fn from_matrix(matrix: [[bool; 3]; 3]) -> Self {
        debug_assert_eq!(Self::WIDTH, 3);
        debug_assert_eq!(Self::HEIGHT, 3);

        Self::from_binary([
            (matrix[0][0] as u8) << 2 | (matrix[0][1] as u8) << 1 | (matrix[0][2] as u8),
            (matrix[1][0] as u8) << 2 | (matrix[1][1] as u8) << 1 | (matrix[1][2] as u8),
            (matrix[2][0] as u8) << 2 | (matrix[2][1] as u8) << 1 | (matrix[2][2] as u8),
        ])
    }

    pub fn to_matrix(self) -> [[bool; 3]; 3] {
        debug_assert_eq!(Self::WIDTH, 3);
        debug_assert_eq!(Self::HEIGHT, 3);

        let mut matrix = [[false; 3]; 3];
        #[allow(clippy::needless_range_loop)]
        for x in 0..Self::WIDTH {
            for y in 0..Self::HEIGHT {
                matrix[y][x] = self.get(x, y);
            }
        }
        matrix
    }

    pub fn get(self, x: usize, y: usize) -> bool {
        debug_assert!(x < Self::WIDTH);
        debug_assert!(y < Self::HEIGHT);

        let index = y * Self::WIDTH + x;
        self.0 & (1 << index) != 0
    }

    pub fn get_neighbours(self) -> ShapeNeighbours {
        NEIGHBOUR_CACHE.get_or_init(|| {
            // technically we're allocating more than we need, since we'll always work with
            // normalized shapes. the overhead of making this array holey or using a PHF is not
            // worth it though.
            let mut cache = [ShapeNeighbours::default(); Self::MAX_SHAPE_COUNT as usize];
            for (i, cache_item) in cache.iter_mut().enumerate() {
                *cache_item = Self(i as u16).calculate_neighbours();
            }
            cache
        })[self.0 as usize]
    }

    pub fn apply_transformation(self, transformation: Transformation) -> Self {
        self.apply_raw_transformation(transformation).normalize()
    }

    /// Apply a transformation without normalizing the shape afterwards.
    fn apply_raw_transformation(self, transformation: Transformation) -> Self {
        debug_assert_eq!(Self::WIDTH, 3);
        debug_assert_eq!(Self::HEIGHT, 3);

        match transformation {
            Transformation::FlipHorizontal => Self(
                self.0 & 0b010_010_010
                    | (self.0 & 0b100_100_100) >> 2
                    | (self.0 & 0b001_001_001) << 2,
            ),
            Transformation::FlipVertical => Self(
                self.0 & 0b000_111_000
                    | (self.0 & 0b111_000_000) >> (2 * Self::WIDTH)
                    | (self.0 & 0b000_000_111) << (2 * Self::WIDTH),
            ),
            Transformation::Rotate90 => {
                let shape_bits = self.0;
                let bit = |bit: usize| (shape_bits & (1 << bit)) >> bit;

                Self(
                    bit(6)
                        | bit(3) << 1
                        | bit(0) << 2
                        | bit(7) << 3
                        | bit(4) << 4 // technically `shape_bits & 0b000_010_000`
                        | bit(1) << 5
                        | bit(8) << 6
                        | bit(5) << 7
                        | bit(2) << 8,
                )
            }
            // I can probably optimize this by doing the proper bit shifting
            Transformation::Rotate180 => self
                .apply_raw_transformation(Transformation::Rotate90)
                .apply_raw_transformation(Transformation::Rotate90),
            Transformation::Rotate270 => self
                .apply_raw_transformation(Transformation::Rotate90)
                .apply_raw_transformation(Transformation::Rotate90)
                .apply_raw_transformation(Transformation::Rotate90),
        }
    }

    /// Normalize a shape by aligning it to the top left corner.
    pub fn normalize(mut self) -> Self {
        debug_assert_eq!(Self::WIDTH, 3);
        debug_assert_eq!(Self::HEIGHT, 3);

        // move the shape left if it does not touch the left edge
        for _ in 0..(Self::WIDTH - 1) {
            if self.0 & 0b001_001_001 == 0 {
                self.0 >>= 1;
            }
        }

        // move the shape up if it does not touch the top edge
        for _ in 0..(Self::HEIGHT - 1) {
            if self.0 & 0b000_000_111 == 0 {
                self.0 >>= Self::WIDTH;
            }
        }

        debug_assert!(self.is_normalized());

        self
    }

    /// Checks if a shape is aligned to the top left corner.
    const fn is_normalized(self) -> bool {
        let touches_left = self.0 & 0b001_001_001 != 0;
        let touches_top = self.0 & 0b000_000_111 != 0;
        self.0 == 0 || (touches_left && touches_top)
    }

    fn calculate_neighbours(self) -> ShapeNeighbours {
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

    pub fn get_max_x(self) -> usize {
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

    pub fn get_max_y(self) -> usize {
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

    pub fn to_braille(self) -> String {
        debug_assert_eq!(Self::WIDTH, 3);
        debug_assert_eq!(Self::HEIGHT, 3);

        // shuffle bits around to match braille indices
        // our input is currently mapped like this:
        // 0 1 2
        // 3 4 5
        // 6 7 8
        // we want to map our bits so that they become 2 braille characters, so they should look like this:
        // | 0 1 | 2 . |
        // | 3 4 | 5 . |
        // | 6 7 | 8 . |
        // | . . | . . |
        // braille maps indices like this:
        // 0 3
        // 1 4
        // 2 5
        // 6 7
        // see unicode chart for more info: https://www.unicode.org/charts/PDF/U2800.pdf
        // we'll map 1 byte to each character, so we need to shuffle the bits around a bit
        let shape_bits = self.0;
        let bit = |bit_idx: u16| ((shape_bits & (1 << bit_idx)) >> bit_idx) as u32;

        #[allow(clippy::identity_op)]
        let bits = bit(0) << 0
            | bit(1) << 3
            | bit(2) << (8 + 0)
            | bit(3) << 1
            | bit(4) << 4
            | bit(5) << (8 + 1)
            | bit(6) << 2
            | bit(7) << 5
            | bit(8) << (8 + 2);

        let mut result = String::with_capacity(2);
        result.push(char::from_u32(0x2800 + (bits & 0xFF)).unwrap());
        result.push(char::from_u32(0x2800 + ((bits >> 8) & 0xFF)).unwrap());

        result
    }
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_braille())
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
    fn test_to_braille() {
        assert_eq!(
            "⠐⠀",
            format!("{:?}", Shape::from_binary([0b000, 0b010, 0b000]))
        );
        assert_eq!(
            "⠿⠇",
            format!("{:?}", Shape::from_binary([0b111, 0b111, 0b111]))
        );
        assert_eq!(
            "⠮⠂",
            format!("{:?}", Shape::from_binary([0b010, 0b101, 0b110]))
        );
    }

    #[test]
    fn test_get_coordinate() {
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

    #[test]
    fn test_get_max_x_y() {
        assert_eq!(0, Shape::from_binary([0b100, 0b000, 0b000]).get_max_x());
        assert_eq!(0, Shape::from_binary([0b100, 0b000, 0b000]).get_max_y());
        assert_eq!(1, Shape::from_binary([0b000, 0b010, 0b000]).get_max_x());
        assert_eq!(1, Shape::from_binary([0b000, 0b010, 0b000]).get_max_y());
        assert_eq!(2, Shape::from_binary([0b111, 0b111, 0b111]).get_max_x());
        assert_eq!(2, Shape::from_binary([0b111, 0b111, 0b111]).get_max_y());
        assert_eq!(2, Shape::from_binary([0b010, 0b101, 0b010]).get_max_x());
        assert_eq!(2, Shape::from_binary([0b010, 0b101, 0b010]).get_max_y());

        assert_eq!(0, Shape::from_binary([0b000, 0b000, 0b100]).get_max_x());
        assert_eq!(2, Shape::from_binary([0b000, 0b000, 0b100]).get_max_y());

        assert_eq!(2, Shape::from_binary([0b001, 0b000, 0b000]).get_max_x());
        assert_eq!(0, Shape::from_binary([0b001, 0b000, 0b000]).get_max_y());
    }

    #[test]
    fn test_is_normalized() {
        assert!(Shape::from_binary([0b111, 0b111, 0b111]).is_normalized());
        assert!(!Shape::from_binary([0b000, 0b010, 0b000]).is_normalized());

        assert!(Shape::from_binary([0b000, 0b000, 0b000]).is_normalized());

        assert!(Shape::from_binary([0b100, 0b000, 0b000]).is_normalized());
        assert!(Shape::from_binary([0b111, 0b000, 0b000]).is_normalized());
        assert!(Shape::from_binary([0b100, 0b100, 0b100]).is_normalized());
        assert!(Shape::from_binary([0b100, 0b000, 0b001]).is_normalized());
        assert!(Shape::from_binary([0b001, 0b000, 0b100]).is_normalized());

        assert!(!Shape::from_binary([0b001, 0b000, 0b000]).is_normalized());
        assert!(!Shape::from_binary([0b000, 0b000, 0b100]).is_normalized());
        assert!(!Shape::from_binary([0b000, 0b111, 0b000]).is_normalized());
        assert!(!Shape::from_binary([0b010, 0b010, 0b010]).is_normalized());
        assert!(!Shape::from_binary([0b000, 0b010, 0b001]).is_normalized());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(
            Shape::from_binary([0b000, 0b010, 0b000]).normalize(),
            Shape::from_binary([0b100, 0b000, 0b000]),
        );
        assert_eq!(
            Shape::from_binary([0b001, 0b000, 0b000]).normalize(),
            Shape::from_binary([0b100, 0b000, 0b000]),
        );
        assert_eq!(
            Shape::from_binary([0b000, 0b000, 0b100]).normalize(),
            Shape::from_binary([0b100, 0b000, 0b000]),
        );
        assert_eq!(
            Shape::from_binary([0b000, 0b111, 0b000]).normalize(),
            Shape::from_binary([0b111, 0b000, 0b000]),
        );
        assert_eq!(
            Shape::from_binary([0b010, 0b010, 0b010]).normalize(),
            Shape::from_binary([0b100, 0b100, 0b100]),
        );
        assert_eq!(
            Shape::from_binary([0b000, 0b010, 0b001]).normalize(),
            Shape::from_binary([0b100, 0b010, 0b000]),
        );
    }

    #[test]
    fn test_raw_transformation_rotate_90() {
        assert_eq!(
            Shape::from_binary([0b010, 0b010, 0b010])
                .apply_raw_transformation(Transformation::Rotate90),
            Shape::from_binary([0b000, 0b111, 0b000]),
        );
        assert_eq!(
            Shape::from_binary([0b010, 0b110, 0b000])
                .apply_raw_transformation(Transformation::Rotate90),
            Shape::from_binary([0b010, 0b011, 0b000]),
        );
        assert_eq!(
            Shape::from_binary([0b100, 0b000, 0b010])
                .apply_raw_transformation(Transformation::Rotate90),
            Shape::from_binary([0b001, 0b100, 0b000]),
        );
    }

    #[test]
    fn test_raw_transformation_rotate_combine() {
        for i in 0..Shape::MAX_SHAPE_COUNT {
            assert_eq!(
                Shape(i).apply_raw_transformation(Transformation::Rotate180),
                Shape(i)
                    .apply_raw_transformation(Transformation::Rotate90)
                    .apply_raw_transformation(Transformation::Rotate90),
            );
            assert_eq!(
                Shape(i).apply_raw_transformation(Transformation::Rotate270),
                Shape(i)
                    .apply_raw_transformation(Transformation::Rotate90)
                    .apply_raw_transformation(Transformation::Rotate90)
                    .apply_raw_transformation(Transformation::Rotate90),
            );
            assert_eq!(
                Shape(i),
                Shape(i)
                    .apply_raw_transformation(Transformation::Rotate90)
                    .apply_raw_transformation(Transformation::Rotate90)
                    .apply_raw_transformation(Transformation::Rotate90)
                    .apply_raw_transformation(Transformation::Rotate90),
            );
        }
    }

    #[test]
    fn test_raw_transformation_flip() {
        assert_eq!(
            Shape::from_binary([0b100, 0b001, 0b010])
                .apply_raw_transformation(Transformation::FlipHorizontal),
            Shape::from_binary([0b001, 0b100, 0b010]),
        );
        assert_eq!(
            Shape::from_binary([0b100, 0b001, 0b010])
                .apply_raw_transformation(Transformation::FlipVertical),
            Shape::from_binary([0b010, 0b001, 0b100]),
        );

        assert_eq!(
            Shape::from_binary([0b000, 0b000, 0b000])
                .apply_raw_transformation(Transformation::FlipHorizontal),
            Shape::from_binary([0b000, 0b000, 0b000]),
        );
        assert_eq!(
            Shape::from_binary([0b111, 0b111, 0b111])
                .apply_raw_transformation(Transformation::FlipVertical),
            Shape::from_binary([0b111, 0b111, 0b111]),
        );
    }

    #[test]
    fn test_raw_transformation_flip_twice() {
        for i in 0..Shape::MAX_SHAPE_COUNT {
            assert_eq!(
                Shape(i),
                Shape(i)
                    .apply_raw_transformation(Transformation::FlipVertical)
                    .apply_raw_transformation(Transformation::FlipVertical),
            );
            assert_eq!(
                Shape(i),
                Shape(i)
                    .apply_raw_transformation(Transformation::FlipHorizontal)
                    .apply_raw_transformation(Transformation::FlipHorizontal),
            );
        }
    }

    #[test]
    fn test_raw_transformation_no_bit_loss() {
        for i in 0..Shape::MAX_SHAPE_COUNT {
            assert_eq!(
                i.count_ones(),
                Shape(i)
                    .apply_raw_transformation(Transformation::Rotate90)
                    .apply_raw_transformation(Transformation::Rotate180)
                    .apply_raw_transformation(Transformation::Rotate270)
                    .apply_raw_transformation(Transformation::FlipVertical)
                    .apply_raw_transformation(Transformation::FlipHorizontal)
                    .0
                    .count_ones(),
            );
        }
    }
}
