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

    pub fn update(&self, generation_time: f32) -> f32 {
        match self.kind {
            InputNeuronKind::Random => self.unit_to_range(rand::random::<f32>()),
            InputNeuronKind::Time => self.unit_to_range(generation_time),
        }
    }

    pub fn kind(&self) -> InputNeuronKind {
        self.kind
    }

    fn unit_to_range(&self, unit: f32) -> f32 {
        unit * (self.signal_range * 2.0) - self.signal_range
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
        1
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
