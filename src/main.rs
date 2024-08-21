mod entity;
mod life_sim;
mod neural_network;
mod settings;
mod util;

use cellular_automata::sim::run_sim;

use life_sim::LifeSim;
use pixels::Error;
use settings::Settings;

fn main() -> Result<(), Error> {
    let settings = Settings::new().unwrap();
    let sim = Box::new(LifeSim::new(&settings));

    run_sim(sim)
}
