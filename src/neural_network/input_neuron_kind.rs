use std::fmt;

#[derive(Clone, Copy, Debug)]
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
