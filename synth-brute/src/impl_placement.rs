use crate::*;

impl Placement {
    pub fn new(index: usize, transformations: ()) -> Self {
        Self {
            index,
            transformations,
        }
    }
}
