use std::collections::HashMap;

use strum::IntoEnumIterator;

use super::input_neuron_kind::InputNeuronKind;
use super::output_neuron_kind::OutputNeuronKind;

#[derive(Debug)]
pub struct Brain {
    pub input_layer: Vec<InputNeuronKind>,
    pub output_layer: Vec<OutputNeuronKind>,
    pub connections: HashMap<(usize, usize), f32>,
    pub output_fire_threshold: f32,
}

impl Brain {
    pub fn new(connection_count: usize, output_fire_threshold: f32) -> Self {
        let mut connections = HashMap::<(usize, usize), f32>::new();

        // Create random connections from input to output.
        for _ in 0..connection_count {
            let mut input;
            let mut output;

            // between -1.0 and 1.0.
            let weight = (rand::random::<f32>() - 0.5) * 2.0;

            loop {
                input = rand::random::<usize>() % InputNeuronKind::iter().count();
                output = rand::random::<usize>() % OutputNeuronKind::iter().count();

                if connections.contains_key(&(input, output)) {
                    continue;
                } else {
                    break;
                }
            }

            connections.insert((input, output), weight);
        }

        Self {
            input_layer: InputNeuronKind::iter().collect(),
            output_layer: OutputNeuronKind::iter().collect(),
            connections,
            output_fire_threshold,
        }
    }

    pub fn decide(
        &mut self,
        generation_time: f32,
        danger_dist: (u32, u32),
        grid_size: (u32, u32),
    ) -> Vec<OutputNeuronKind> {
        let mut decisions = Vec::new();

        let mut output_signals = vec![0.0; self.output_layer.len()];
        for ((input_index, output_index), weight) in &self.connections {
            let input = self.input_layer[*input_index];

            let raw_signal = match input {
                InputNeuronKind::Random => rand::random::<f32>(),
                InputNeuronKind::Time => generation_time,
                InputNeuronKind::DangerDistX => danger_dist.0 as f32 / grid_size.0 as f32,
                InputNeuronKind::DangerDistY => danger_dist.1 as f32 / grid_size.1 as f32,
            };

            output_signals[*output_index] += raw_signal * weight;
        }

        // Check if each output neuron should fire.
        for (i, output_kind) in &mut self.output_layer.iter().enumerate() {
            let signal = output_signals[i];

            if signal.tanh() >= self.output_fire_threshold {
                decisions.push(output_kind.clone());
            }
        }

        decisions
    }
}

impl Clone for Brain {
    fn clone(&self) -> Self {
        Self {
            input_layer: self.input_layer.clone(),
            output_layer: self.output_layer.clone(),
            connections: self.connections.clone(),
            output_fire_threshold: self.output_fire_threshold,
        }
    }
}
