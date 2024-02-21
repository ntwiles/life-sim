use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Evolution {}

impl Evolution {}

#[derive(Debug, Deserialize)]
pub struct Settings {
    entity_child_count: usize,
    grid_width: u32,
    grid_height: u32,
    neuron_connection_count: usize,
    neuron_fire_threshold: f32,
    neuron_signal_range: f32,
    render_killzone_color: [u8; 4],
    render_pixel_scale: u32,
    entity_start_count: u32,
    sim_generation_steps: usize,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("default"))
            // TODO: Find out out to make this optional.
            // .add_source(config::File::with_name("settings"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }

    pub fn entity_child_count(&self) -> usize {
        self.entity_child_count
    }

    pub fn entity_start_count(&self) -> u32 {
        self.entity_start_count
    }

    pub fn grid_width(&self) -> u32 {
        self.grid_width
    }

    pub fn grid_height(&self) -> u32 {
        self.grid_height
    }

    pub fn neuron_connection_count(&self) -> usize {
        self.neuron_connection_count
    }

    pub fn neuron_fire_threshold(&self) -> f32 {
        self.neuron_fire_threshold
    }

    pub fn neuron_signal_range(&self) -> f32 {
        self.neuron_signal_range
    }

    pub fn render_killzone_color(&self) -> [u8; 4] {
        self.render_killzone_color
    }

    pub fn render_pixel_scale(&self) -> u32 {
        self.render_pixel_scale
    }

    pub fn sim_generation_steps(&self) -> usize {
        self.sim_generation_steps
    }
}
