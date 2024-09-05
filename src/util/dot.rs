use dot_writer::{Attributes, DotWriter};

use crate::neural_network::brain::Brain;
use crate::neural_network::connection::Connection;
use crate::neural_network::neuron_kind::NeuronKind;

fn get_neuron_label(neuron: &NeuronKind, index: u16) -> String {
    match neuron {
        NeuronKind::Input(input) => format!("{}_{}", input, index),
        NeuronKind::Hidden(hidden) => format!("{}_{}", hidden, index),
        NeuronKind::Output(output) => format!("{}_{}", output, index),
    }
}

fn neural_net_to_dot(brain: &Brain) -> String {
    let mut bytes = Vec::new();
    let mut writer = DotWriter::from(&mut bytes);

    writer.set_pretty_print(false);

    let mut graph = writer.digraph();

    for Connection {
        source,
        target,
        weight,
    } in &brain.connections
    {
        let a_label = get_neuron_label(&brain.neurons[*source as usize], *source);
        let b_label = get_neuron_label(&brain.neurons[*target as usize], *target);

        graph
            .edge_attributes()
            .set("label", &format!("{:.2}", weight), false);

        graph.edge(a_label, b_label);
    }

    drop(graph);

    std::str::from_utf8(&bytes).unwrap().to_string()
}

pub fn write_dot_file(brain: &Brain, id: usize) {
    let dot = neural_net_to_dot(brain);

    std::fs::write(format!("./data/dots/{}.dot", id), dot).unwrap();
}
