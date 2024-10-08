use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Evolution {}

impl Evolution {}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub entity_start_count: u32,
    pub entity_survivor_child_count: u32,
    pub entity_survivor_breed_rate: f32,

    pub grid_width: u32,
    pub grid_height: u32,

    pub neural_network_hidden_layer_width: usize,
    pub neural_network_hidden_layer_depth: usize,

    pub neural_network_mutation_rate: f32,

    pub render_rad_zone_color: [u8; 4],
    pub render_background_color: [u8; 4],
    pub render_pixel_scale: u32,

    pub debug: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("default"))
            // TODO: Find out how to make this optional.
            // .add_source(config::File::with_name("settings"))
            .build()?;

        s.try_deserialize()
    }
}
