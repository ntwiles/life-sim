use std::fmt;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumIter, EnumCount, PartialEq)]
pub enum OutputNeuron {
    MoveRandom,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Stay,
}

impl fmt::Display for OutputNeuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputNeuron::MoveRandom => write!(f, "MoveRandom"),
            OutputNeuron::MoveUp => write!(f, "MoveUp"),
            OutputNeuron::MoveDown => write!(f, "MoveDown"),
            OutputNeuron::MoveLeft => write!(f, "MoveLeft"),
            OutputNeuron::MoveRight => write!(f, "MoveRight"),
            OutputNeuron::Stay => write!(f, "Stay"),
        }
    }
}
