use std::collections::HashMap;

use cellular_automata::{
    automata::Automata,
    grid::grid_coords_to_index,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

use colorgrad::Gradient;

use crate::entity::Entity;
use crate::kill_zone::{is_point_in_killzone, KillZone};
use crate::neural_network::brain::Brain;
use crate::neural_network::output_neuron_kind::OutputNeuronKind;
use crate::settings::Settings;
use crate::util::dot::neural_net_to_dot;

pub struct LifeSim {
    entity_child_count: usize,
    entities: Vec<Entity>,

    kill_zones: Vec<KillZone>,

    grid_width: u32,
    grid_height: u32,

    neuron_connection_count: usize,

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
        let neuron_output_fire_threshold = settings.neuron_fire_threshold();
        let neuron_signal_range = settings.neuron_signal_range();

        let mut entities = Vec::new();
        let used_positions = Vec::new();

        for _ in 0..settings.entity_start_count() {
            let (x, y) = get_random_position(&used_positions, grid_width, grid_height);

            let brain = Brain::new(
                neuron_connection_count,
                neuron_signal_range,
                neuron_output_fire_threshold,
            );

            let entity = Entity::new(x, y, brain);

            entities.push(entity);
        }

        let kill_zones = vec![
            KillZone {
                start_time: 0.1,
                end_time: 0.2,
                position: (120, 0),
                width: 30,
                height: grid_height,
            },
            KillZone {
                start_time: 0.2,
                end_time: 0.3,
                position: (90, 0),
                width: 30,
                height: grid_height,
            },
            KillZone {
                start_time: 0.3,
                end_time: 0.4,
                position: (60, 0),
                width: 30,
                height: grid_height,
            },
            KillZone {
                start_time: 0.4,
                end_time: 0.5,
                position: (30, 0),
                width: 30,
                height: 30,
            },
            KillZone {
                start_time: 0.4,
                end_time: 0.5,
                position: (30, 120),
                width: 30,
                height: 30,
            },
            KillZone {
                start_time: 0.5,
                end_time: 0.7,
                position: (0, 0),
                width: 30,
                height: 30,
            },
            KillZone {
                start_time: 0.5,
                end_time: 0.7,
                position: (0, 120),
                width: 30,
                height: 30,
            },
            KillZone {
                start_time: 0.7,
                end_time: 0.9,
                position: (0, 0),
                width: 30,
                height: grid_height,
            },
        ];

        let render_color_gradient = colorgrad::rainbow();

        Self {
            entity_child_count: settings.entity_child_count(),
            entities,
            kill_zones,
            grid_width,
            grid_height,
            neuron_connection_count,
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

type RenderContext = (f32, HashMap<usize, [u8; 4]>);

impl Automata<RenderContext> for LifeSim {
    fn update(&mut self) {
        let generation_time = self.sim_current_step as f32 / self.sim_generation_steps as f32;

        for entity in &mut self.entities {
            if !entity.is_alive() {
                continue;
            }

            if is_point_in_killzone(&self.kill_zones, (entity.x(), entity.y()), generation_time) {
                entity.kill();
            } else {
                entity.update(self.grid_width, self.grid_height, generation_time);
            }
        }

        if self.sim_current_step > self.sim_generation_steps {
            let selected = self.entities.iter().filter(|e| e.is_alive());

            let mut next_generation = Vec::new();
            let used_positions = Vec::new();

            println!("\nGeneration completed. Survivors:");

            for entity in selected {
                for _ in 0..self.entity_child_count {
                    let (x, y) =
                        get_random_position(&used_positions, self.grid_width, self.grid_height);

                    let brain = entity.brain().clone();

                    let dot = neural_net_to_dot(&brain);
                    println!("{}", dot);

                    let child = Entity::new(x, y, brain);
                    next_generation.push(child);
                }
            }

            println!("\n");

            self.entities = next_generation;
            self.sim_current_step = 0;
        } else {
            self.sim_current_step += 1;
        }
    }

    fn before_render(&self) -> RenderContext {
        let entity_colors = get_entity_colors(
            &self.entities,
            &self.render_color_gradient,
            self.grid_width,
            self.neuron_connection_count,
        );

        let generation_time = self.sim_current_step as f32 / self.sim_generation_steps as f32;

        (generation_time, entity_colors)
    }

    fn render(&self, context: &RenderContext, i: usize, pixel: &mut [u8]) {
        let (generation_time, entity_colors) = context;
        let (vx, vy) =
            viewport_index_to_coords(i, self.render_viewport_width, self.render_viewport_height);
        let (x, y) = viewport_to_grid(vx, vy, self.render_pixel_scale);
        let index = grid_coords_to_index(x, y, self.grid_width);

        let color: [u8; 4] = if entity_colors.contains_key(&index) {
            entity_colors[&index]
        } else if is_point_in_killzone(&self.kill_zones, (x, y), *generation_time) {
            self.render_killzone_color
        } else {
            [0x0, 0x0, 0x0, 0xff]
        };

        pixel.copy_from_slice(&color);
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

fn get_entity_colors(
    entities: &Vec<Entity>,
    render_color_gradient: &Gradient,
    grid_width: u32,
    neuron_connection_count: usize,
) -> HashMap<usize, [u8; 4]> {
    let mut entity_colors: HashMap<usize, [u8; 4]> = HashMap::new();

    for entity in entities {
        if !entity.is_alive() {
            continue;
        }

        let index = grid_coords_to_index(entity.x(), entity.y(), grid_width);

        let output_sum = entity
            .brain()
            .connections
            .iter()
            .fold(0, |acc, ((_, v), _)| acc + v);

        let max_sum = OutputNeuronKind::count() * neuron_connection_count;

        let color_index: f64 = output_sum as f64 / max_sum as f64;
        let color = render_color_gradient.at(color_index).to_rgba8();

        entity_colors.insert(index, color);
    }

    entity_colors
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
