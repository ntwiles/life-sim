mod body;
mod entities;
mod entity_config;
mod grid_config;
mod life_sim;
mod neural_network;
mod neural_network_config;
mod render_config;
mod rendering;
mod scenario;
mod settings;
mod util;

use cellular_automata::sim::run_sim;

use entity_config::EntityConfig;
use grid_config::GridConfig;
use life_sim::LifeSim;
use neural_network_config::NeuralNetworkConfig;
use pixels::Error;
use render_config::RenderConfig;
use scenario::{kill_zone::KillZone, scenario::Scenario};
use settings::Settings;

fn main() -> Result<(), Error> {
    let settings = Settings::new().unwrap();

    let render_config = RenderConfig {
        pixel_scale: settings.render_pixel_scale,
        killzone_color: settings.render_killzone_color,
        background_color: settings.render_background_color,
        viewport_width: settings.render_pixel_scale * settings.grid_width,
        viewport_height: settings.render_pixel_scale * settings.grid_height,
        color_gradient: colorgrad::rainbow(),
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

        connection_mutation_rate: settings.neural_network_connection_mutation_rate,
        connection_mutation_magnitude: settings.neural_network_connection_mutation_magnitude,

        structural_mutation_rate: settings.neural_network_structural_mutation_rate,
    };

    let kill_zones = vec![
        KillZone {
            start_time: 30,
            end_time: 60,
            position: (120, 0),
            width: 30,
            height: grid_config.height,
        },
        KillZone {
            start_time: 60,
            end_time: 90,
            position: (90, 0),
            width: 30,
            height: grid_config.height,
        },
        KillZone {
            start_time: 90,
            end_time: 120,
            position: (60, 0),
            width: 30,
            height: grid_config.height,
        },
        KillZone {
            start_time: 120,
            end_time: 150,
            position: (30, 0),
            width: 30,
            height: 30,
        },
        KillZone {
            start_time: 120,
            end_time: 150,
            position: (30, 120),
            width: 30,
            height: 30,
        },
        KillZone {
            start_time: 150,
            end_time: 180,
            position: (0, 0),
            width: 30,
            height: 30,
        },
        KillZone {
            start_time: 150,
            end_time: 180,
            position: (0, 120),
            width: 30,
            height: 30,
        },
        KillZone {
            start_time: 180,
            end_time: 210,
            position: (0, 0),
            width: 30,
            height: grid_config.height,
        },
    ];

    let scenario = Scenario::new(kill_zones);

    let sim = Box::new(LifeSim::new(
        scenario,
        grid_config,
        render_config,
        entity_config,
        network_config,
    ));

    run_sim(sim)
}
