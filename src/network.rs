use std::collections::HashSet;

use super::input_neuron::{InputNeuron, InputNeuronKind};
use super::output_neuron::{OutputNeuron, OutputNeuronKind};

// TODO: Pull this out of the evolution module so it can be used elsewhere.
#[derive(Debug)]
pub struct NeuralNetwork {
    pub input_layer: Vec<InputNeuron>,
    pub output_layer: Vec<OutputNeuron>,
    pub connections: HashSet<(usize, usize)>,
}

impl NeuralNetwork {
    pub fn new(
        connections: HashSet<(usize, usize)>,
        neuron_signal_range: f32,
        neuron_fire_threshold: f32,
    ) -> Self {
        Self {
            input_layer: vec![InputNeuron::new(
                InputNeuronKind::Random,
                neuron_signal_range,
            )],
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

    pub fn decide(&self) -> Vec<OutputNeuronKind> {
        let mut decisions = Vec::new();

        for (i, input) in self.input_layer.iter().enumerate() {
            // Don't bother updating inputs that aren't connected to anything.
            if !self.connections.iter().any(|(input, _)| *input == i) {
                continue;
            }

            let signal = input.update();

            let connected_outputs = self
                .connections
                .iter()
                .filter(|(input, _)| *input == i)
                .map(|(_, output)| &self.output_layer[*output]);

            for output in connected_outputs {
                if output.update(signal) {
                    decisions.push(output.kind());
                }
            }
        }

        decisions
    }
}
