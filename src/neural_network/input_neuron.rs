use std::fmt;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumIter, EnumCount, PartialEq)]
pub enum InputNeuron {
    Random,
    PreviousMoveDirSin,
    PreviousMoveDirCos,
    Time,
    DangerDist,
    DangerDirSin,
    DangerDirCos,
}

impl fmt::Display for InputNeuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputNeuron::Random => write!(f, "Random"),
            InputNeuron::PreviousMoveDirSin => write!(f, "PreviousMoveDirSin"),
            InputNeuron::PreviousMoveDirCos => write!(f, "PreviousMoveDirCos"),
            InputNeuron::Time => write!(f, "Time"),
            InputNeuron::DangerDist => write!(f, "DangerDist"),
            InputNeuron::DangerDirSin => write!(f, "DangerDirSin"),
            InputNeuron::DangerDirCos => write!(f, "DangerDirCos"),
        }
    }
}
