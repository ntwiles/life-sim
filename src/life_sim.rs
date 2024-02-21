use std::collections::{HashMap, HashSet};

use cellular_automata::{
    automata::Automata,
    grid::grid_coords_to_index,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

use colorgrad::Gradient;

use crate::dot::neural_net_to_dot;
use crate::entity::Entity;
use crate::input_neuron::InputNeuronKind;
use crate::network::NeuralNetwork;
use crate::output_neuron::OutputNeuronKind;
use crate::settings::Settings;

pub struct LifeSim {
    entity_child_count: usize,
    entities: Vec<Entity>,

    grid_width: u32,
    grid_height: u32,

    neuron_connection_count: usize,
    neuron_fire_threshold: f32,
    neuron_signal_range: f32,

    render_pixel_scale: u32,
    render_color_gradient: Gradient,
    render_viewport_width: u32,
    render_viewport_height: u32,
    render_killzone_color: [u8; 4],

    sim_current_step: usize,
    sim_generation_steps: usize,
}

impl LifeSim {
    pub fn new(settings: &Settings) -> Self {
        let grid_width = settings.grid_width();
        let grid_height = settings.grid_height();
        let neuron_connection_count = settings.neuron_connection_count();
        let neuron_fire_threshold = settings.neuron_fire_threshold();
        let neuron_signal_range = settings.neuron_signal_range();

        let mut entities = Vec::new();
        let used_positions = Vec::new();

        for _ in 0..settings.entity_start_count() {
            let (x, y) = get_random_position(&used_positions, grid_width, grid_height);

            let mut connections = HashSet::<(usize, usize)>::new();

            // Create random connections from input to output.
            for _ in 0..neuron_connection_count {
                let mut input;
                let mut output;

                loop {
                    input = rand::random::<usize>() % InputNeuronKind::count();
                    output = rand::random::<usize>() % OutputNeuronKind::count();

                    if connections.contains(&(input, output)) {
                        continue;
                    } else {
                        break;
                    }
                }

                connections.insert((input, output));
            }

            let brain = NeuralNetwork::new(connections, neuron_signal_range, neuron_fire_threshold);
            let entity = Entity::new(x, y, brain);

            entities.push(entity);
        }

        let render_color_gradient = colorgrad::rainbow();

        Self {
            entity_child_count: settings.entity_child_count(),
            entities,
            grid_width,
            grid_height,
            neuron_connection_count,
            neuron_fire_threshold,
            neuron_signal_range,
            render_pixel_scale: settings.render_pixel_scale(),
            render_color_gradient,
            sim_current_step: 0,
            sim_generation_steps: settings.sim_generation_steps(),
            render_killzone_color: settings.render_killzone_color(),
            render_viewport_width: settings.render_pixel_scale() * settings.grid_width(),
            render_viewport_height: settings.render_pixel_scale() * settings.grid_height(),
        }
    }
}

impl Automata for LifeSim {
    fn update(&mut self) {
        let generation_time = self.sim_current_step as f32 / self.sim_generation_steps as f32;

        for entity in &mut self.entities {
            if is_in_killzone(self.grid_width, entity.x, generation_time) {
                entity.kill();
            } else {
                entity.update(self.grid_width, self.grid_height, generation_time);
            }
        }

        if self.sim_current_step > self.sim_generation_steps {
            let selected = self.entities.iter().filter(|e| {
                // Select if entity is in right 30% of grid and is alive.
                e.is_alive() && e.x > self.grid_width - (self.grid_width as f32 * 0.3) as u32
            });

            let mut next_generation = Vec::new();
            let used_positions = Vec::new();

            for entity in selected {
                for _ in 0..self.entity_child_count {
                    let (x, y) =
                        get_random_position(&used_positions, self.grid_width, self.grid_height);

                    let brain = NeuralNetwork::new(
                        entity.brain.connections.clone(),
                        self.neuron_signal_range,
                        self.neuron_fire_threshold,
                    );

                    let dot = neural_net_to_dot(&brain);
                    println!("{}", dot);

                    let child = Entity::new(x, y, brain);
                    next_generation.push(child);
                }
            }

            self.entities = next_generation;
            self.sim_current_step = 0;
        } else {
            self.sim_current_step += 1;
        }
    }

    fn render(&self, pixels: &mut [u8]) {
        let mut entity_colors: HashMap<usize, [u8; 4]> = HashMap::new();

        for entity in &self.entities {
            if !entity.is_alive() {
                continue;
            }

            let index = grid_coords_to_index(entity.x, entity.y, self.grid_width);

            let output_sum = entity
                .brain
                .connections
                .iter()
                .fold(0, |acc, (_, v)| acc + v);

            let max_sum = OutputNeuronKind::count() * self.neuron_connection_count;

            let color_index: f64 = output_sum as f64 / max_sum as f64;
            let color = self.render_color_gradient.at(color_index).to_rgba8();

            entity_colors.insert(index, color);
        }

        let generation_time = self.sim_current_step as f32 / self.sim_generation_steps as f32;

        for (i, pixel) in pixels.chunks_exact_mut(4).enumerate() {
            let (vx, vy) = viewport_index_to_coords(
                i,
                self.render_viewport_width,
                self.render_viewport_height,
            );
            let (x, y) = viewport_to_grid(vx, vy, self.render_pixel_scale);
            let index = grid_coords_to_index(x, y, self.grid_width);

            let color: [u8; 4] = if entity_colors.contains_key(&index) {
                entity_colors[&index]
            } else if is_in_killzone(self.grid_width, x, generation_time) {
                self.render_killzone_color
            } else {
                [0x0, 0x0, 0x0, 0xff]
            };

            pixel.copy_from_slice(&color);
        }
    }

    fn grid_width(&self) -> u32 {
        self.grid_width
    }

    fn grid_height(&self) -> u32 {
        self.grid_height
    }

    fn render_pixel_scale(&self) -> u32 {
        self.render_pixel_scale
    }
}

fn get_random_position(used_positions: &[u32], grid_width: u32, grid_height: u32) -> (u32, u32) {
    loop {
        let x = rand::random::<u32>() % grid_width;
        let y = rand::random::<u32>() % grid_height;

        let index = grid_coords_to_index(x, y, grid_width);

        if !used_positions.contains(&(index as u32)) {
            return (x, y);
        }
    }
}

fn is_in_killzone(grid_width: u32, x: u32, generation_time: f32) -> bool {
    generation_time < 0.5 && x > (grid_width as f32 * 0.6) as u32
}
