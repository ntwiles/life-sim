use std::fmt;
#[derive(Clone, Copy, Debug)]
pub struct InputNeuron {
    kind: InputNeuronKind,
}

impl InputNeuron {
    pub fn new(kind: InputNeuronKind) -> Self {
        Self { kind }
    }

    pub fn update(&self, generation_time: f32) -> f32 {
        match self.kind {
            InputNeuronKind::Random => rand::random::<f32>(),
            InputNeuronKind::Time => generation_time,
        }
    }

    pub fn kind(&self) -> InputNeuronKind {
        self.kind
    }
}

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
