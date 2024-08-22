use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum OutputNeuronKind {
    MoveRandom,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Stay,
}

impl OutputNeuronKind {
    pub fn count() -> usize {
        6
    }
}

impl fmt::Display for OutputNeuronKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputNeuronKind::MoveRandom => write!(f, "MoveRandom"),
            OutputNeuronKind::MoveUp => write!(f, "MoveUp"),
            OutputNeuronKind::MoveDown => write!(f, "MoveDown"),
            OutputNeuronKind::MoveLeft => write!(f, "MoveLeft"),
            OutputNeuronKind::MoveRight => write!(f, "MoveRight"),
            OutputNeuronKind::Stay => write!(f, "Stay"),
        }
    }
}
