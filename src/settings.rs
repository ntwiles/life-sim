use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Evolution {}

impl Evolution {}

#[derive(Debug, Deserialize)]
pub struct Settings {
    entity_start_count: u32,
    entity_child_count: u32,
    entity_mutation_rate: f32,
    entity_mutation_magnitude: f32,

    grid_width: u32,
    grid_height: u32,

    neuron_hidden_layer_width: usize,

    render_killzone_color: [u8; 4],
    render_pixel_scale: u32,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("default"))
            // TODO: Find out how to make this optional.
            // .add_source(config::File::with_name("settings"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }

    pub fn entity_child_count(&self) -> u32 {
        self.entity_child_count
    }

    pub fn entity_start_count(&self) -> u32 {
        self.entity_start_count
    }

    pub fn entity_mutation_rate(&self) -> f32 {
        self.entity_mutation_rate
    }

    pub fn entity_mutation_magnitude(&self) -> f32 {
        self.entity_mutation_magnitude
    }

    pub fn grid_width(&self) -> u32 {
        self.grid_width
    }

    pub fn grid_height(&self) -> u32 {
        self.grid_height
    }

    pub fn neuron_hidden_layer_width(&self) -> usize {
        self.neuron_hidden_layer_width
    }

    pub fn render_killzone_color(&self) -> [u8; 4] {
        self.render_killzone_color
    }

    pub fn render_pixel_scale(&self) -> u32 {
        self.render_pixel_scale
    }
}
