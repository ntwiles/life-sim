use std::collections::HashMap;

use crate::genome::gene::Gene;

use super::connection::Connection;
use super::hidden_neuron::HiddenNeuron;
use super::input_neuron::InputNeuron;
use super::neuron_kind::NeuronKind;
use super::output_neuron::OutputNeuron;

#[derive(Debug)]
pub struct Brain {
    pub neurons: Vec<NeuronKind>,
    pub connections: Vec<Connection>,
    pub genome: Vec<Gene>,

    previous_move: OutputNeuron,
    input_neurons: Vec<u16>,
    output_neurons: Vec<u16>,
}

impl Brain {
    pub fn from_genome(genome: Vec<Gene>) -> Self {
        let mut neurons = Vec::new();
        let mut input_neurons = Vec::new();
        let mut output_neurons = Vec::new();
        let mut connections = Vec::new();

        let mut input_instance = HashMap::<(u16, u16), u16>::new();
        let mut hidden_instances = HashMap::<(u16, u16), u16>::new();
        let mut output_instances = HashMap::<(u16, u16), u16>::new();

        for gene in &genome {
            let Gene {
                source_discriminant,
                source_is_hidden,
                source_instance,
                target_discriminant,
                target_is_output,
                target_instance,
                weight,
            } = gene;

            let source_instances = if *source_is_hidden {
                &mut hidden_instances
            } else {
                &mut input_instance
            };

            let key = (*source_discriminant, *source_instance);

            let source_index = if source_instances.contains_key(&key) {
                *source_instances.get(&key).unwrap()
            } else {
                let source_index = neurons.len() as u16;
                source_instances.insert(key, source_index);

                if *source_is_hidden {
                    neurons.push(NeuronKind::Hidden(HiddenNeuron::from_discriminant(
                        *source_discriminant as usize,
                    )));
                } else {
                    input_neurons.push(source_index);
                    neurons.push(NeuronKind::Input(InputNeuron::from_discriminant(
                        *source_discriminant as usize,
                    )));
                };

                source_index
            };

            let target_instances = if *target_is_output {
                &mut output_instances
            } else {
                &mut hidden_instances
            };

            let key = (*target_discriminant, *target_instance);

            let target_index = if target_instances.contains_key(&key) {
                *target_instances.get(&key).unwrap()
            } else {
                let target_index = neurons.len() as u16;
                target_instances.insert(key, target_index);

                if *target_is_output {
                    output_neurons.push(target_index);
                    neurons.push(NeuronKind::Output(OutputNeuron::from_discriminant(
                        *target_discriminant as usize,
                    )));
                } else {
                    neurons.push(NeuronKind::Hidden(HiddenNeuron::from_discriminant(
                        *target_discriminant as usize,
                    )));
                };

                target_index
            };

            let connection = Connection {
                source: source_index,
                target: target_index,
                weight: *weight,
            };

            connections.push(connection)
        }

        Self {
            genome,
            neurons,
            connections,
            input_neurons,
            output_neurons,
            previous_move: OutputNeuron::MoveRight,
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
            let input = &self.neurons[*input_index as usize];

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

            signals[*input_index as usize] = raw_signal;
        }

        // Propogate signals through the network.
        for Connection {
            source,
            target,
            weight,
        } in &self.connections
        {
            let signal = signals[*source as usize];

            match &self.neurons[*target as usize] {
                NeuronKind::Hidden(hidden) => {
                    let hidden = match hidden {
                        HiddenNeuron::Identity => signal,
                        HiddenNeuron::Gaussian => (-signal.powi(2) / 2.0).exp(),
                        HiddenNeuron::Sigmoid => 1.0 / (1.0 + std::f32::consts::E.powf(-signal)),
                        HiddenNeuron::ReLU => signal.max(0.0),
                        HiddenNeuron::Tanh => signal.tanh(),
                    };

                    signals[*target as usize] += hidden * weight;
                }
                NeuronKind::Output(_) => {}
                _ => panic!("Connections should end in hidden or output neurons."),
            }

            signals[*target as usize] += signal * weight;
        }

        let mut decision = (f32::NEG_INFINITY, OutputNeuron::MoveRandom);

        // Check if each output neuron should fire.
        for output_index in &self.output_neurons {
            let output = match self.neurons[*output_index as usize] {
                NeuronKind::Output(output) => output,
                _ => panic!("Output layer should only contain output neurons."),
            };

            let signal = signals[*output_index as usize].tanh();

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
            output_neurons: self.output_neurons.clone(),
            connections: self.connections.clone(),
            genome: self.genome.clone(),
            neurons: self.neurons.clone(),
            previous_move: OutputNeuron::MoveRight,
        }
    }
}
