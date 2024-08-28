use std::fmt;

use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumIter, EnumCount)]
pub enum HiddenNeuron {
    Identity,
    Gaussian,
    ReLU,
    Sigmoid,
    Tanh,
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
