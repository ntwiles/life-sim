pub mod gene;
pub mod mutation;

use std::collections::HashMap;

use gene::Gene;
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;

use crate::{
    neural_network::{
        hidden_neuron::HiddenNeuron, input_neuron::InputNeuron, output_neuron::OutputNeuron,
    },
    neural_network_config::NeuralNetworkConfig,
};

pub fn random_genome(network_config: &NeuralNetworkConfig) -> Vec<Gene> {
    let NeuralNetworkConfig {
        hidden_layer_width,
        hidden_layer_depth,
        ..
    } = network_config;
    let mut genome = Vec::new();

    let mut rng = rand::thread_rng();

    let mut input_layer = Vec::new();

    let mut hidden_discriminant_instances = HashMap::<u16, u16>::new();
    let mut output_discriminant_instances = HashMap::<u16, u16>::new();

    // Create random connections from input to hidden.
    for _ in 0..*hidden_layer_width {
        let input_discriminant =
            InputNeuron::iter().choose(&mut rng).unwrap().discriminant() as u16;

        let hidden_discriminant = HiddenNeuron::iter()
            .choose(&mut rng)
            .unwrap()
            .discriminant() as u16;

        // between -1.0 and 1.0.
        let weight = (rand::random::<f32>() - 0.5) * 2.0;

        let target_instance = *hidden_discriminant_instances
            .entry(hidden_discriminant)
            .and_modify(|e| *e += 1)
            .or_insert(0);

        let gene = Gene {
            source_discriminant: input_discriminant,
            source_is_hidden: false,
            source_instance: 0,
            target_discriminant: hidden_discriminant,
            target_is_output: false,
            target_instance,
            weight,
        };

        input_layer.push((gene.target_discriminant, gene.target_instance));
        genome.push(gene);
    }

    let mut prev_layer = input_layer;

    // Create random connections from hidden to hidden.
    for _ in 0..*hidden_layer_depth - 1 {
        let mut hidden_layer = Vec::new();

        for (source_discriminant, source_instance) in prev_layer {
            let mut rng = rand::thread_rng();
            let hidden_discriminant = HiddenNeuron::iter()
                .choose(&mut rng)
                .unwrap()
                .discriminant() as u16;

            // between -1.0 and 1.0.
            let weight = (rand::random::<f32>() - 0.5) * 2.0;

            let target_instance = *hidden_discriminant_instances
                .entry(hidden_discriminant)
                .and_modify(|e| *e += 1)
                .or_insert(0);

            let gene = Gene {
                source_discriminant,
                source_is_hidden: true,
                source_instance,
                target_discriminant: hidden_discriminant,
                target_is_output: false,
                target_instance,
                weight,
            };

            hidden_layer.push((gene.target_discriminant, gene.target_instance));
            genome.push(gene);
        }

        prev_layer = hidden_layer;
    }

    // Create random connections from hidden to output.
    for (source_discriminant, source_instance) in prev_layer {
        let output_discriminant = OutputNeuron::iter()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .discriminant() as u16;

        // between -1.0 and 1.0.
        let weight = (rand::random::<f32>() - 0.5) * 2.0;

        let target_instance = *output_discriminant_instances
            .entry(output_discriminant)
            .and_modify(|e| *e += 1)
            .or_insert(0);

        let gene = Gene {
            source_discriminant,
            source_instance,
            source_is_hidden: true,
            target_discriminant: output_discriminant,
            target_is_output: true,
            target_instance,
            weight,
        };

        genome.push(gene);
    }

    genome
}
