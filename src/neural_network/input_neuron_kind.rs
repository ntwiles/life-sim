use std::fmt;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumIter, EnumCount)]
pub enum InputNeuronKind {
    Random,
    Time,
}

// TODO: This is really janky, find a more idiomatic solution.
impl InputNeuronKind {
    pub fn count() -> usize {
        2
    }
}

impl fmt::Display for InputNeuronKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputNeuronKind::Random => write!(f, "Random"),
            InputNeuronKind::Time => write!(f, "Time"),
        }
    }
}
