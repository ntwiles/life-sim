mod body;
mod entities;
mod entity_config;
mod grid_config;
mod kill_zone;
mod life_sim;
mod neural_network;
mod neural_network_config;
mod render_config;
mod settings;
mod util;

use cellular_automata::sim::run_sim;

use entity_config::EntityConfig;
use grid_config::GridConfig;
use life_sim::LifeSim;
use neural_network_config::NeuralNetworkConfig;
use pixels::Error;
use render_config::RenderConfig;
use settings::Settings;

fn main() -> Result<(), Error> {
    let settings = Settings::new().unwrap();

    let render_config = RenderConfig {
        pixel_scale: settings.render_pixel_scale(),
        killzone_color: settings.render_killzone_color(),
        viewport_width: settings.render_pixel_scale() * settings.grid_width(),
        viewport_height: settings.render_pixel_scale() * settings.grid_height(),
        color_gradient: colorgrad::rainbow(),
    };

    let entity_config = EntityConfig {
        child_count: settings.entity_child_count(),
        start_count: settings.entity_start_count(),
        mutation_rate: settings.entity_mutation_rate(),
        mutation_magnitude: settings.entity_mutation_magnitude(),
    };

    let grid_config = GridConfig {
        width: settings.grid_width(),
        height: settings.grid_height(),
    };

    let network_config = NeuralNetworkConfig {
        hidden_layer_width: settings.neuron_hidden_layer_width(),
    };

    let sim = Box::new(LifeSim::new(
        grid_config,
        render_config,
        entity_config,
        network_config,
    ));

    run_sim(sim)
}
