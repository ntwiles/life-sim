use std::fmt;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumIter, EnumCount)]
pub enum InputNeuronKind {
    Random,
    Time,
    ProximityToDanger,
}

impl fmt::Display for InputNeuronKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputNeuronKind::Random => write!(f, "Random"),
            InputNeuronKind::Time => write!(f, "Time"),
            InputNeuronKind::ProximityToDanger => write!(f, "ProximityToDanger"),
        }
    }
}
