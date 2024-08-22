use std::collections::HashMap;

use super::input_neuron_kind::InputNeuronKind;
use super::output_neuron::{OutputNeuron, OutputNeuronKind};

#[derive(Debug)]
pub struct Brain {
    pub input_layer: Vec<InputNeuronKind>,
    pub output_layer: Vec<OutputNeuron>,
    pub connections: HashMap<(usize, usize), f32>,
}

impl Brain {
    pub fn new(connections: HashMap<(usize, usize), f32>, neuron_fire_threshold: f32) -> Self {
        Self {
            input_layer: vec![InputNeuronKind::Random, InputNeuronKind::Time],
            output_layer: vec![
                OutputNeuron::new(OutputNeuronKind::MoveRandom, neuron_fire_threshold),
                OutputNeuron::new(OutputNeuronKind::MoveUp, neuron_fire_threshold),
                OutputNeuron::new(OutputNeuronKind::MoveDown, neuron_fire_threshold),
                OutputNeuron::new(OutputNeuronKind::MoveLeft, neuron_fire_threshold),
                OutputNeuron::new(OutputNeuronKind::MoveRight, neuron_fire_threshold),
                OutputNeuron::new(OutputNeuronKind::Stay, neuron_fire_threshold),
            ],
            connections,
        }
    }

    pub fn decide(&mut self, generation_time: f32) -> Vec<OutputNeuronKind> {
        let mut decisions = Vec::new();

        let mut output_signals = vec![0.0; self.output_layer.len()];
        for (i, input_kind) in self.input_layer.iter().enumerate() {
            let raw_signal = match input_kind {
                InputNeuronKind::Random => rand::random::<f32>(),
                InputNeuronKind::Time => generation_time,
            };

            for ((input, output), weight) in &self.connections {
                if *input != i {
                    continue;
                }

                output_signals[*output] += raw_signal * weight;
            }
        }

        for (i, output) in &mut self.output_layer.iter().enumerate() {
            let signal = output_signals[i];

            if signal.tanh() >= output.fire_threshold {
                decisions.push(output.kind());
            }
        }

        decisions
    }
}
