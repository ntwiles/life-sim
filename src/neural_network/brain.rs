use rand::seq::IteratorRandom;

use strum::IntoEnumIterator;

use super::hidden_neuron::HiddenNeuron;
use super::input_neuron::InputNeuron;
use super::output_neuron::OutputNeuron;

#[derive(Debug, Clone)]
enum NeuronKind {
    Input(InputNeuron),
    Hidden(HiddenNeuron),
    Output(OutputNeuron),
}

#[derive(Debug)]
pub struct Brain {
    connections: Vec<(usize, usize, f32)>,
    output_fire_threshold: f32,
    neurons: Vec<NeuronKind>,
    input_layer: Vec<usize>,
    output_layer: Vec<usize>,
}

impl Brain {
    pub fn new(hidden_neuron_width: usize, output_fire_threshold: f32) -> Self {
        let mut connections = Vec::<(usize, usize, f32)>::new();
        let mut neurons = Vec::new();

        let mut input_layer = Vec::new();

        for input in InputNeuron::iter() {
            let input_index = neurons.len();
            neurons.push(NeuronKind::Input(input));
            input_layer.push(input_index);
        }

        let mut hidden_layer = Vec::new();

        // create random connections from input to hidden.
        for _ in 0..hidden_neuron_width {
            let input_index = input_layer[rand::random::<usize>() % input_layer.len()];

            let mut rng = rand::thread_rng();
            let hidden = NeuronKind::Hidden(HiddenNeuron::iter().choose(&mut rng).unwrap());

            let hidden_index = neurons.len();
            neurons.push(hidden);

            hidden_layer.push(hidden_index);

            // between -1.0 and 1.0.
            let weight = (rand::random::<f32>() - 0.5) * 2.0;

            connections.push((input_index, hidden_index, weight));
        }

        let mut output_layer = Vec::new();

        // Create random connections from hidden to output.
        for prev_index in hidden_layer {
            let output = NeuronKind::Output(
                OutputNeuron::iter()
                    .choose(&mut rand::thread_rng())
                    .unwrap(),
            );

            let output_index = neurons.len();
            neurons.push(output);
            output_layer.push(output_index);

            // between -1.0 and 1.0.
            let weight = (rand::random::<f32>() - 0.5) * 2.0;

            connections.push((prev_index, output_index, weight));
        }

        Self {
            // input_layer: InputNeuron::iter().collect(),
            // hidden_layer: HiddenNeuron::iter().collect(),
            // output_layer: OutputNeuron::iter().collect(),
            connections,
            output_fire_threshold,
            input_layer,
            output_layer,
            neurons,
        }
    }

    pub fn decide(
        &mut self,
        generation_time: f32,
        danger_dist: (u32, u32),
        grid_size: (u32, u32),
    ) -> Vec<OutputNeuron> {
        let mut signals = vec![0.0; self.neurons.len()];

        // Initialize input signals.
        for input_index in &self.input_layer {
            let input = &self.neurons[*input_index];

            let raw_signal: f32 = match input {
                NeuronKind::Input(input) => match input {
                    InputNeuron::Random => rand::random::<f32>(),
                    InputNeuron::Time => generation_time,
                    InputNeuron::DangerDistX => danger_dist.0 as f32 / grid_size.0 as f32,
                    InputNeuron::DangerDistY => danger_dist.1 as f32 / grid_size.1 as f32,
                },
                _ => panic!("Input layer should only contain input neurons."),
            };

            signals[*input_index] = raw_signal;
        }

        // Propogate signals through the network.
        for (a_index, b_index, weight) in &self.connections {
            let signal = signals[*a_index];

            match &self.neurons[*b_index] {
                NeuronKind::Hidden(hidden) => {
                    let hidden = match hidden {
                        HiddenNeuron::Identity => signal,
                        HiddenNeuron::Inverse => 1.0 - signal,
                    };

                    signals[*b_index] += hidden * weight;
                }
                NeuronKind::Output(_) => {}
                _ => panic!("Connections should end in hidden or output neurons."),
            }

            signals[*b_index] += signal * weight;
        }

        let mut decisions = Vec::new();

        // Check if each output neuron should fire.
        for output_index in &self.output_layer {
            let output = match self.neurons[*output_index] {
                NeuronKind::Output(output) => output,
                _ => panic!("Output layer should only contain output neurons."),
            };

            let signal = signals[*output_index];

            if signal.tanh() >= self.output_fire_threshold {
                decisions.push(output.clone());
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
            neurons: self.neurons.clone(),
        }
    }
}
