use super::{hidden_neuron::HiddenNeuron, input_neuron::InputNeuron, output_neuron::OutputNeuron};

#[derive(Debug, Clone)]
pub enum NeuronKind {
    Input(InputNeuron),
    Hidden(HiddenNeuron),
    Output(OutputNeuron),
}
