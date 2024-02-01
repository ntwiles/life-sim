mod dot;
mod entity;
mod input_neuron;
mod life_sim;
mod network;
mod output_neuron;
mod settings;

use cellular_automata::sim::run_sim;

use life_sim::LifeSim;
use pixels::Error;
use settings::Settings;

fn main() -> Result<(), Error> {
    let settings = Settings::new().unwrap();
    let sim = Box::new(LifeSim::new(&settings));

    run_sim(sim)
}
