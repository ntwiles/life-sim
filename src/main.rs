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
use scenario::{kill_zone::KillZone, scenario::Scenario};
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
    };

    // TODO: Find a better way to persist these values, maybe a JSON file?
    // let kill_zones = vec![
    //     KillZone {
    //         start_time: 30,
    //         end_time: 60,
    //         position: (120, 0),
    //         width: 31,
    //         height: grid_config.height,
    //     },
    //     KillZone {
    //         start_time: 60,
    //         end_time: 90,
    //         position: (90, 0),
    //         width: 31,
    //         height: grid_config.height,
    //     },
    //     KillZone {
    //         start_time: 90,
    //         end_time: 120,
    //         position: (60, 0),
    //         width: 31,
    //         height: grid_config.height,
    //     },
    //     KillZone {
    //         start_time: 120,
    //         end_time: 150,
    //         position: (30, 0),
    //         width: 31,
    //         height: 30,
    //     },
    //     KillZone {
    //         start_time: 120,
    //         end_time: 150,
    //         position: (30, 120),
    //         width: 31,
    //         height: 30,
    //     },
    //     KillZone {
    //         start_time: 150,
    //         end_time: 180,
    //         position: (0, 0),
    //         width: 31,
    //         height: 30,
    //     },
    //     KillZone {
    //         start_time: 150,
    //         end_time: 180,
    //         position: (0, 120),
    //         width: 31,
    //         height: 30,
    //     },
    //     KillZone {
    //         start_time: 180,
    //         end_time: 210,
    //         position: (0, 0),
    //         width: 31,
    //         height: grid_config.height,
    //     },
    // ];

    let kill_zones = vec![
        KillZone {
            start_time: 0,
            end_time: 300,
            position: (25, 25),
            width: 30,
            height: 30,
        },
        KillZone {
            start_time: 0,
            end_time: 300,
            position: (85, 85),
            width: 40,
            height: 40,
        },
    ];

    let food_amount = settings.scenario_starting_food_amount;

    // let scenario = Scenario::new(
    //     kill_zones,
    //     food_amount,
    //     grid_config.width,
    //     grid_config.height,
    //     true,
    //     false,
    // );

    let scenario_file = load_scenario("buffet").unwrap();
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
