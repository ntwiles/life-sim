use std::fmt;

use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumIter, EnumCount, PartialEq)]
pub enum HiddenNeuron {
    Identity,
    Gaussian,
    ReLU,
    Sigmoid,
    Tanh,
}

impl HiddenNeuron {
    pub fn from_discriminant(discriminant: usize) -> Self {
        let discriminant = discriminant % HiddenNeuron::iter().count();

        match discriminant {
            0 => HiddenNeuron::Identity,
            1 => HiddenNeuron::Gaussian,
            2 => HiddenNeuron::ReLU,
            3 => HiddenNeuron::Sigmoid,
            4 => HiddenNeuron::Tanh,
            _ => panic!("Invalid discriminant for HiddenNeuron: {}", discriminant),
        }
    }
    pub fn discriminant(&self) -> usize {
        match self {
            HiddenNeuron::Identity => 0,
            HiddenNeuron::Gaussian => 1,
            HiddenNeuron::ReLU => 2,
            HiddenNeuron::Sigmoid => 3,
            HiddenNeuron::Tanh => 4,
        }
    }
}

impl fmt::Display for HiddenNeuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HiddenNeuron::Identity => write!(f, "Identity"),
            HiddenNeuron::Gaussian => write!(f, "Gaussian"),
            HiddenNeuron::ReLU => write!(f, "ReLU"),
            HiddenNeuron::Sigmoid => write!(f, "Sigmoid"),
            HiddenNeuron::Tanh => write!(f, "Tanh"),
        }
    }
}
