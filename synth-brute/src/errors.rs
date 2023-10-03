use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum SynthError {
    /// A tile was placed out of bounds
    OutOfBounds,
    /// A tile was placed on top of another tile while it is disallowed
    DisallowedOverlap,
}

impl Display for SynthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds => write!(f, "A tile was placed out of bounds"),
            Self::DisallowedOverlap => write!(
                f,
                "A tile was placed on top of another tile while it is disallowed"
            ),
        }
    }
}

impl Error for SynthError {}
