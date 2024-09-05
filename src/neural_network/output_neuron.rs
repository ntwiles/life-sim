use std::fmt;
use strum::IntoEnumIterator;
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

impl OutputNeuron {
    pub fn from_discriminant(discriminant: usize) -> Self {
        let discriminant = discriminant % OutputNeuron::iter().count();

        match discriminant {
            0 => OutputNeuron::MoveRandom,
            1 => OutputNeuron::MoveUp,
            2 => OutputNeuron::MoveDown,
            3 => OutputNeuron::MoveLeft,
            4 => OutputNeuron::MoveRight,
            5 => OutputNeuron::Stay,
            _ => panic!("Invalid discriminant for OutputNeuron: {}", discriminant),
        }
    }
    pub fn discriminant(&self) -> usize {
        match self {
            OutputNeuron::MoveRandom => 0,
            OutputNeuron::MoveUp => 1,
            OutputNeuron::MoveDown => 2,
            OutputNeuron::MoveLeft => 3,
            OutputNeuron::MoveRight => 4,
            OutputNeuron::Stay => 5,
        }
    }
}

impl fmt::Display for OutputNeuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TODO: Consider removing random move.
            OutputNeuron::MoveRandom => write!(f, "MoveRandom"),
            OutputNeuron::MoveUp => write!(f, "MoveUp"),
            OutputNeuron::MoveDown => write!(f, "MoveDown"),
            OutputNeuron::MoveLeft => write!(f, "MoveLeft"),
            OutputNeuron::MoveRight => write!(f, "MoveRight"),
            OutputNeuron::Stay => write!(f, "Stay"),
        }
    }
}
