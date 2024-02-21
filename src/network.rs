use std::collections::HashSet;

use super::input_neuron::{InputNeuron, InputNeuronKind};
use super::output_neuron::{OutputNeuron, OutputNeuronKind};

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
        let mut fired_outputs = Vec::new();

        for (i, input) in self.input_layer.iter().enumerate() {
            // TODO: filter + map -> filter_map or fold
            let connected_outputs: Vec<&OutputNeuron> = self
                .connections
                .iter()
                .filter(|(input, _)| *input == i)
                .map(|(_, output)| &self.output_layer[*output])
                .collect();

            if connected_outputs.len() == 0 {
                continue;
            }

            let signal = input.update();

            for output in connected_outputs {
                if output.update(signal) {
                    fired_outputs.push(output.kind());
                }
            }
        }

        fired_outputs
    }
}
