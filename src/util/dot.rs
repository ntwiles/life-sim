use dot_writer::DotWriter;

use crate::neural_network::brain::Brain;

pub fn neural_net_to_dot(brain: &Brain) -> String {
    let mut bytes = Vec::new();
    let mut writer = DotWriter::from(&mut bytes);

    writer.set_pretty_print(false);

    {
        let mut graph = writer.digraph();

        for (i, input_kind) in brain.input_layer.iter().enumerate() {
            let connections = brain
                .connections
                .iter()
                .filter(|((input, _), _)| *input == i);

            for connection in connections {
                let output = &brain.output_layer[connection.0 .1];
                graph.edge(input_kind.to_string(), output.kind().to_string());
            }
        }
    }

    std::str::from_utf8(&bytes).unwrap().to_string()
}
