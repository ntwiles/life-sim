use rand::seq::{IteratorRandom, SliceRandom};

use strum::IntoEnumIterator;

use crate::neural_network_config::NeuralNetworkConfig;

use super::hidden_neuron::HiddenNeuron;
use super::input_neuron::InputNeuron;
use super::neuron_kind::NeuronKind;
use super::output_neuron::OutputNeuron;

#[derive(Debug)]
pub struct Brain {
    pub neurons: Vec<NeuronKind>,
    pub connections: Vec<(usize, usize, f32)>,

    previous_move: OutputNeuron,
    input_neurons: Vec<usize>,
    hidden_neurons: Vec<usize>,
    output_neurons: Vec<usize>,
}

impl Brain {
    pub fn new(hidden_neuron_width: usize, hidden_neuron_depth: usize) -> Self {
        let mut neurons = Vec::new();
        let mut connections = Vec::<(usize, usize, f32)>::new();

        let mut input_neurons = Vec::new();

        for input in InputNeuron::iter() {
            let input_index = neurons.len();
            neurons.push(NeuronKind::Input(input));
            input_neurons.push(input_index);
        }

        let mut current_hidden_layer = Vec::new();

        // Create random connections from input to hidden.
        for _ in 0..hidden_neuron_width {
            let input_index = input_neurons[rand::random::<usize>() % input_neurons.len()];

            let mut rng = rand::thread_rng();
            let hidden = NeuronKind::Hidden(HiddenNeuron::iter().choose(&mut rng).unwrap());

            let hidden_index = neurons.len();
            neurons.push(hidden);

            current_hidden_layer.push(hidden_index);

            // between -1.0 and 1.0.
            let weight = (rand::random::<f32>() - 0.5) * 2.0;

            connections.push((input_index, hidden_index, weight));
        }

        let mut hidden_neurons = current_hidden_layer.clone();
        let mut prev_hidden_layer = current_hidden_layer;

        current_hidden_layer = Vec::new();

        // Create random connections from hidden to hidden.
        for _ in 0..hidden_neuron_depth - 1 {
            for prev_index in &prev_hidden_layer {
                let mut rng = rand::thread_rng();
                let hidden = NeuronKind::Hidden(HiddenNeuron::iter().choose(&mut rng).unwrap());

                let hidden_index = neurons.len();
                neurons.push(hidden);
                current_hidden_layer.push(hidden_index);

                // between -1.0 and 1.0.
                let weight = (rand::random::<f32>() - 0.5) * 2.0;

                connections.push((*prev_index, hidden_index, weight));
            }

            hidden_neurons.extend(&current_hidden_layer.clone());
            prev_hidden_layer = current_hidden_layer;
            current_hidden_layer = Vec::new();
        }

        let mut output_neurons = Vec::new();

        // Create random connections from hidden to output.
        for prev_index in prev_hidden_layer {
            let output = NeuronKind::Output(
                OutputNeuron::iter()
                    .choose(&mut rand::thread_rng())
                    .unwrap(),
            );

            let output_index = neurons.len();
            neurons.push(output);
            output_neurons.push(output_index);

            // between -1.0 and 1.0.
            let weight = (rand::random::<f32>() - 0.5) * 2.0;

            connections.push((prev_index, output_index, weight));
        }

        Self {
            neurons,
            connections,
            input_neurons,
            hidden_neurons,
            output_neurons,
            previous_move: OutputNeuron::MoveRight,
        }
    }

    pub fn mutate_connections(&mut self, network_config: &NeuralNetworkConfig) {
        let num_to_mutate =
            (self.connections.len() as f32 * network_config.connection_mutation_rate) as usize;

        for _ in 0..num_to_mutate {
            let mutation_amount =
                (rand::random::<f32>() - 0.5) * 2.0 * network_config.connection_mutation_magnitude;

            let connection = self
                .connections
                .choose_mut(&mut rand::thread_rng())
                .unwrap();

            connection.2 += mutation_amount;
            connection.2 = connection.2.max(-1.0).min(1.0);
        }
    }

    pub fn mutate_structure(&mut self, network_config: &NeuralNetworkConfig) {
        let num_to_mutate =
            (self.connections.len() as f32 * network_config.structure_mutation_rate) as usize;

        let mut rng = rand::thread_rng();

        for _ in 0..num_to_mutate {
            let existing = self.hidden_neurons[rand::random::<usize>() % self.hidden_neurons.len()];

            loop {
                let new_type = NeuronKind::Hidden(HiddenNeuron::iter().choose(&mut rng).unwrap());

                if new_type != self.neurons[existing] {
                    self.neurons[existing] = new_type;
                    break;
                }
            }

            let a_index = self.neurons.len() - 2;
            let b_index = self.neurons.len() - 1;

            let weight = (rand::random::<f32>() - 0.5) * 2.0;

            self.connections.push((a_index, b_index, weight));
        }
    }

    pub fn decide(
        &mut self,
        generation_time: f32,
        danger_dist: f32,
        danger_dir_sin: f32,
        danger_dir_cos: f32,
    ) -> OutputNeuron {
        let mut signals = vec![0.0; self.neurons.len()];

        // Initialize input signals.
        for input_index in &self.input_neurons {
            let input = &self.neurons[*input_index];

            let raw_signal: f32 = match input {
                NeuronKind::Input(input) => match input {
                    InputNeuron::Random => rand::random::<f32>(),
                    InputNeuron::PreviousMoveDirCos => match self.previous_move {
                        OutputNeuron::MoveLeft => -1.0,
                        OutputNeuron::MoveRight => 1.0,
                        OutputNeuron::MoveUp => 0.0,
                        OutputNeuron::MoveDown => 0.0,
                        OutputNeuron::Stay => 0.0,
                        OutputNeuron::MoveRandom => 0.0,
                    },
                    InputNeuron::PreviousMoveDirSin => match self.previous_move {
                        OutputNeuron::MoveLeft => 0.0,
                        OutputNeuron::MoveRight => 0.0,
                        OutputNeuron::MoveUp => 1.0,
                        OutputNeuron::MoveDown => -1.0,
                        OutputNeuron::Stay => 0.0,
                        OutputNeuron::MoveRandom => 0.0,
                    },
                    InputNeuron::Time => generation_time,
                    InputNeuron::DangerDist => danger_dist,
                    InputNeuron::DangerDirCos => danger_dir_cos,
                    InputNeuron::DangerDirSin => danger_dir_sin,
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
                        HiddenNeuron::Gaussian => (-signal.powi(2) / 2.0).exp(),
                        HiddenNeuron::Sigmoid => 1.0 / (1.0 + std::f32::consts::E.powf(-signal)),
                        HiddenNeuron::ReLU => signal.max(0.0),
                        HiddenNeuron::Tanh => signal.tanh(),
                    };

                    signals[*b_index] += hidden * weight;
                }
                NeuronKind::Output(_) => {}
                _ => panic!("Connections should end in hidden or output neurons."),
            }

            signals[*b_index] += signal * weight;
        }

        let mut decision = (f32::NEG_INFINITY, OutputNeuron::MoveRandom);

        // Check if each output neuron should fire.
        for output_index in &self.output_neurons {
            let output = match self.neurons[*output_index] {
                NeuronKind::Output(output) => output,
                _ => panic!("Output layer should only contain output neurons."),
            };

            let signal = signals[*output_index].tanh();

            if signal >= decision.0 {
                decision = (signal, output);
            }
        }

        self.previous_move = decision.1;
        self.previous_move
    }
}

impl Clone for Brain {
    fn clone(&self) -> Self {
        Self {
            input_neurons: self.input_neurons.clone(),
            hidden_neurons: self.hidden_neurons.clone(),
            output_neurons: self.output_neurons.clone(),
            connections: self.connections.clone(),
            neurons: self.neurons.clone(),
            previous_move: OutputNeuron::MoveRight,
        }
    }
}
