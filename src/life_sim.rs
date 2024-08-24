use std::collections::HashMap;

use cellular_automata::{
    automata::Automata,
    grid::grid_coords_to_index,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

use crate::kill_zone::{is_point_in_killzone, KillZone};
use crate::neural_network::brain::Brain;
use crate::neural_network::output_neuron::OutputNeuron;
use crate::settings::Settings;
use crate::util::dot::neural_net_to_dot;
use crate::{body::Body, kill_zone::distance_to_killzone};
use colorgrad::Gradient;
use strum::IntoEnumIterator;

pub struct LifeSim {
    entity_child_count: usize,
    entities: Vec<(Brain, Body)>,

    kill_zones: Vec<KillZone>,

    grid_width: u32,
    grid_height: u32,

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

        let neuron_output_fire_threshold = settings.neuron_fire_threshold();
        let neuron_hidden_layer_width = settings.neuron_hidden_layer_width();

        let mut entities = Vec::new();
        let used_positions = Vec::new();

        for _ in 0..settings.entity_start_count() {
            let (x, y) = get_random_position(&used_positions, grid_width, grid_height);

            let brain = Brain::new(neuron_hidden_layer_width, neuron_output_fire_threshold);
            let body = Body::new(x, y);

            entities.push((brain, body));
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
                end_time: 1.0,
                position: (0, 0),
                width: 30,
                height: grid_height,
            },
        ];

        Self {
            entity_child_count: settings.entity_child_count(),
            entities,
            kill_zones,
            grid_width,
            grid_height,
            render_pixel_scale: settings.render_pixel_scale(),
            render_color_gradient: colorgrad::rainbow(),
            sim_current_step: 0,
            sim_generation_steps: settings.sim_generation_steps(),
            render_killzone_color: settings.render_killzone_color(),
            render_viewport_width: settings.render_pixel_scale() * settings.grid_width(),
            render_viewport_height: settings.render_pixel_scale() * settings.grid_height(),
        }
    }
}

type RenderContext = (f32, HashMap<usize, [u8; 4]>, Vec<KillZone>);

impl Automata<RenderContext> for LifeSim {
    fn update(&mut self) {
        let generation_time = self.sim_current_step as f32 / self.sim_generation_steps as f32;

        let active_kill_zones = self
            .kill_zones
            .iter()
            .filter(|kz| generation_time >= kz.start_time && generation_time <= kz.end_time)
            .collect::<Vec<&KillZone>>();

        for (brain, body) in &mut self.entities {
            if !body.is_alive() {
                continue;
            }

            let danger_dist = distance_to_killzone(&active_kill_zones, (body.x(), body.y()));

            if danger_dist == (0, 0) {
                body.kill();
            } else {
                let grid_size = (self.grid_width, self.grid_height);

                let decisions = brain.decide(generation_time, danger_dist, grid_size);
                body.update(decisions, grid_size);
            }
        }

        if self.sim_current_step > self.sim_generation_steps {
            let selected = self.entities.iter().filter(|(_, body)| body.is_alive());

            let mut next_generation = Vec::new();
            let used_positions = Vec::new();

            println!("\nGeneration completed. Next generation:");

            for (brain, _) in selected {
                for _ in 0..self.entity_child_count {
                    let (x, y) =
                        get_random_position(&used_positions, self.grid_width, self.grid_height);

                    let brain = brain.clone();

                    // TODO: apply both structural and weight mutation.

                    let dot = neural_net_to_dot(&brain);
                    println!("{}", dot);

                    let child = (brain, Body::new(x, y));
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

    // This whole render context dependency injection thing may not be what we want. It might be better to just
    // save state in the automata and have the render function access it directly.
    fn before_render(&self) -> RenderContext {
        let entity_colors =
            get_entity_colors(&self.entities, &self.render_color_gradient, self.grid_width);

        let generation_time = self.sim_current_step as f32 / self.sim_generation_steps as f32;
        let active_kill_zones = self
            .kill_zones
            .clone()
            .into_iter()
            .filter(|kz| generation_time >= kz.start_time && generation_time <= kz.end_time)
            .collect::<Vec<KillZone>>();

        (generation_time, entity_colors, active_kill_zones)
    }

    fn render(&self, context: &RenderContext, i: usize, pixel: &mut [u8]) {
        let (generation_time, entity_colors, active_killzones) = context;
        let (vx, vy) =
            viewport_index_to_coords(i, self.render_viewport_width, self.render_viewport_height);
        let (x, y) = viewport_to_grid(vx, vy, self.render_pixel_scale);
        let index = grid_coords_to_index(x, y, self.grid_width);

        let color: [u8; 4] = if entity_colors.contains_key(&index) {
            entity_colors[&index]
        } else if is_point_in_killzone(active_killzones, (x, y), *generation_time) {
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
    entities: &Vec<(Brain, Body)>,
    render_color_gradient: &Gradient,
    grid_width: u32,
) -> HashMap<usize, [u8; 4]> {
    let mut entity_colors: HashMap<usize, [u8; 4]> = HashMap::new();

    for (brain, body) in entities {
        if !body.is_alive() {
            continue;
        }

        let index = grid_coords_to_index(body.x(), body.y(), grid_width);

        let output_sum = brain.connections.iter().fold(0, |acc, ((_, v, _))| acc + v);

        let max_sum = OutputNeuron::iter().count();

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
