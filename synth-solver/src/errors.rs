use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum SynthError {
    /// A material was placed out of bounds
    OutOfBounds,
    /// A material was placed on top of another material while it is disallowed
    DisallowedOverlap,
    UnavailableTile,
}

impl Display for SynthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds => write!(f, "A material was placed out of bounds"),
            Self::DisallowedOverlap => write!(
                f,
                "A material was placed on top of another material while it is disallowed"
            ),
            Self::UnavailableTile => write!(
                f,
                "A material was placed on an unavailable tile (ie. a hole)"
            ),
        }
    }
}

impl Error for SynthError {}
