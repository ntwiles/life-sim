use std::fmt;
use strum::IntoEnumIterator;
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
    FoodDirSin,
    FoodDirCos,
}

impl InputNeuron {
    pub fn from_discriminant(discriminant: usize) -> Self {
        let discriminant = discriminant % InputNeuron::iter().count();

        match discriminant {
            0 => InputNeuron::Random,
            1 => InputNeuron::PreviousMoveDirSin,
            2 => InputNeuron::PreviousMoveDirCos,
            3 => InputNeuron::Time,
            4 => InputNeuron::DangerDist,
            5 => InputNeuron::DangerDirSin,
            6 => InputNeuron::DangerDirCos,
            7 => InputNeuron::FoodDirSin,
            8 => InputNeuron::FoodDirCos,
            _ => panic!("Invalid discriminant for InputNeuron: {}", discriminant),
        }
    }

    pub fn discriminant(&self) -> usize {
        match self {
            InputNeuron::Random => 0,
            InputNeuron::PreviousMoveDirSin => 1,
            InputNeuron::PreviousMoveDirCos => 2,
            InputNeuron::Time => 3,
            InputNeuron::DangerDist => 4,
            InputNeuron::DangerDirSin => 5,
            InputNeuron::DangerDirCos => 6,
            InputNeuron::FoodDirSin => 7,
            InputNeuron::FoodDirCos => 8,
        }
    }
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
            InputNeuron::FoodDirSin => write!(f, "FoodDirSin"),
            InputNeuron::FoodDirCos => write!(f, "FoodDirCos"),
        }
    }
}
