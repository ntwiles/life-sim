use std::fmt;
#[derive(Clone, Copy, Debug)]
pub struct InputNeuron {
    kind: InputNeuronKind,
    signal_range: f32,
}

impl InputNeuron {
    pub fn new(kind: InputNeuronKind, signal_range: f32) -> Self {
        Self { kind, signal_range }
    }

    pub fn update(&self) -> f32 {
        match self.kind {
            InputNeuronKind::Random => {
                rand::random::<f32>() * (self.signal_range * 2.0) - self.signal_range
            }
        }
    }

    pub fn kind(&self) -> InputNeuronKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InputNeuronKind {
    Random,
}

// TODO: This is really janky, find a more idiomatic solution.
impl InputNeuronKind {
    pub fn count() -> usize {
        1
    }
}

impl fmt::Display for InputNeuronKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputNeuronKind::Random => write!(f, "Random"),
        }
    }
}
