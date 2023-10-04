use crate::{Shape, ShapeNeighbours};

impl ShapeNeighbours {
    const MIN_X: isize = -1;
    const MIN_Y: isize = -1;
    const MAX_X: isize = Self::MIN_X + Self::WIDTH as isize - 1;
    const MAX_Y: isize = Self::MIN_Y + Self::HEIGHT as isize - 1;

    const WIDTH: usize = Shape::WIDTH + 2;
    const HEIGHT: usize = Shape::HEIGHT + 2;

    pub fn set(&mut self, x: isize, y: isize) {
        debug_assert!((Self::MIN_X..=Self::MAX_X).contains(&x));
        debug_assert!((Self::MIN_Y..=Self::MAX_Y).contains(&y));

        let abs_x = (x - Self::MIN_X) as usize;
        let abs_y = (y - Self::MIN_Y) as usize;

        let index = abs_x + abs_y * Self::WIDTH;
        let mask = 1 << index;
        self.0 |= mask;
    }
}

impl IntoIterator for ShapeNeighbours {
    type Item = (isize, isize);
    type IntoIter = ShapeNeighboursIterator;

    fn into_iter(self) -> Self::IntoIter {
        ShapeNeighboursIterator {
            neighbours: self,
            index: 0,
        }
    }
}

pub struct ShapeNeighboursIterator {
    neighbours: ShapeNeighbours,
    index: usize,
}

impl Iterator for ShapeNeighboursIterator {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < ShapeNeighbours::WIDTH * ShapeNeighbours::HEIGHT {
            let mask = 1 << self.index;
            let index = self.index;
            self.index += 1;
            if self.neighbours.0 & mask != 0 {
                let x = (index % ShapeNeighbours::WIDTH) as isize + ShapeNeighbours::MIN_X;
                let y = (index / ShapeNeighbours::WIDTH) as isize + ShapeNeighbours::MIN_Y;
                return Some((x, y));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shape_neighbours() {
        use super::*;

        let neighbours = ShapeNeighbours(0b00000_00000_00000_00000_00000);
        assert_eq!(neighbours.into_iter().count(), 0);

        let neighbours = ShapeNeighbours(0b10000_01000_00110_00100_01111);
        assert_eq!(neighbours.into_iter().count(), 9);

        let neighbours = ShapeNeighbours(0b10000_01000_00110_00111_01011);
        let mut iter = neighbours.into_iter();
        assert_eq!(iter.next(), Some((-1, -1)));
        assert_eq!(iter.next(), Some((0, -1)));
        assert_eq!(iter.next(), Some((2, -1)));
        assert_eq!(iter.next(), Some((-1, 0)));
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), Some((2, 2)));
        assert_eq!(iter.next(), Some((3, 3)));
        assert_eq!(iter.next(), None);
    }
}
