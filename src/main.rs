mod body;
mod entity;
mod entity_config;
mod genome;
mod grid_config;
mod life_sim;
pub mod neural_network;
mod neural_network_config;
mod render_config;
mod rendering;
mod scenario;
mod services;
mod settings;
mod vector_2d;

use cellular_automata::{sim::run_sim, sim_config::SimConfig};

use entity_config::EntityConfig;
use grid_config::GridConfig;
use life_sim::LifeSim;
use neural_network_config::NeuralNetworkConfig;
use pixels::Error;
use render_config::RenderConfig;
use scenario::scenario::Scenario;
use services::scenarios::load_scenario;
use settings::Settings;

fn main() -> Result<(), Error> {
    let settings = Settings::new().unwrap();

    let render_config = RenderConfig {
        pixel_scale: settings.render_pixel_scale,
        killzone_color: settings.render_killzone_color,
        background_color: settings.render_background_color,
        viewport_width: settings.render_pixel_scale * settings.grid_width,
        viewport_height: settings.render_pixel_scale * settings.grid_height,
    };

    let entity_config = EntityConfig {
        child_count: settings.entity_child_count,
        start_count: settings.entity_start_count,
    };

    let grid_config = GridConfig {
        width: settings.grid_width,
        height: settings.grid_height,
    };

    let network_config = NeuralNetworkConfig {
        hidden_layer_width: settings.neural_network_hidden_layer_width,
        hidden_layer_depth: settings.neural_network_hidden_layer_depth,

        mutation_rate: settings.neural_network_mutation_rate,
    };

    let scenario_file = load_scenario("wave").unwrap();
    let scenario = Scenario::from_file(scenario_file, grid_config.width, grid_config.height);

    let sim = Box::new(LifeSim::new(
        scenario,
        grid_config,
        render_config,
        entity_config,
        network_config,
    ));

    run_sim(
        sim,
        Some(SimConfig {
            debug: settings.debug,
        }),
    )
}
