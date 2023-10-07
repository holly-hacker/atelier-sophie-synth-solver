use crate::*;

impl Placement {
    pub fn new(index: usize, transformation: Option<Transformation>) -> Self {
        Self {
            index,
            transformation,
        }
    }
}
