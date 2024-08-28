use std::fmt;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumIter, EnumCount, PartialEq)]
pub enum InputNeuron {
    Random,
    Time,
    DangerDistX,
    DangerDistY,
}

impl fmt::Display for InputNeuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputNeuron::Random => write!(f, "Random"),
            InputNeuron::Time => write!(f, "Time"),
            InputNeuron::DangerDistX => write!(f, "DangerDistX"),
            InputNeuron::DangerDistY => write!(f, "DangerDistY"),
        }
    }
}
