use std::collections::HashMap;

use cellular_automata::{
    automata::Automata,
    grid::grid_coords_to_index,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

use crate::kill_zone::{is_point_in_killzone, KillZone};
use crate::neural_network::brain::Brain;
use crate::settings::Settings;
use crate::{body::Body, kill_zone::distance_to_killzone};
use colorgrad::Gradient;

pub struct LifeSim {
    max_entity_count: u32,
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

    neuron_hidden_layer_width: usize,
    neuron_output_fire_threshold: f32,

    sim_current_step: usize,
    sim_generation_steps: usize,
    sim_generation_number: u32,
}

impl LifeSim {
    pub fn new(settings: &Settings) -> Self {
        let grid_width = settings.grid_width();
        let grid_height = settings.grid_height();

        let neuron_output_fire_threshold = settings.neuron_fire_threshold();
        let neuron_hidden_layer_width = settings.neuron_hidden_layer_width();

        let max_entity_count = settings.entity_start_count();

        let render_color_gradient = colorgrad::rainbow();

        let mut entities = Vec::new();
        let mut used_positions = Vec::<usize>::new();

        for _ in 0..max_entity_count {
            let (brain, body) = spawn_entity(
                Brain::new(neuron_hidden_layer_width, neuron_output_fire_threshold),
                &mut used_positions,
                grid_width,
                grid_height,
                &render_color_gradient,
            );

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
                end_time: 0.6,
                position: (0, 0),
                width: 30,
                height: 30,
            },
            KillZone {
                start_time: 0.5,
                end_time: 0.6,
                position: (0, 120),
                width: 30,
                height: 30,
            },
            KillZone {
                start_time: 0.6,
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
            render_color_gradient,
            render_killzone_color: settings.render_killzone_color(),
            render_viewport_width: settings.render_pixel_scale() * settings.grid_width(),
            render_viewport_height: settings.render_pixel_scale() * settings.grid_height(),
            sim_current_step: 0,
            sim_generation_number: 0,
            sim_generation_steps: settings.sim_generation_steps(),
            max_entity_count,
            neuron_hidden_layer_width,
            neuron_output_fire_threshold,
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
            let selected: Vec<&(Brain, Body)> = self
                .entities
                .iter()
                .filter(|(_, body)| body.is_alive())
                .collect();

            println!(
                "Generation {} over. Survivors {}/{} ({}%)",
                self.sim_generation_number,
                selected.len(),
                self.entities.len(),
                selected.len() as f32 / self.entities.len() as f32 * 100.0
            );

            let mut next_generation = Vec::new();
            let mut used_positions = Vec::<usize>::new();

            // For every slot not taken by a selected entity's children, randomly spawn a new child.
            // This is done to give the population a helping hand, but should eventually be removed in
            // favor of mutation and a better training system.

            let num_selected_children: u32 = std::cmp::min(
                selected.len() as u32 * self.entity_child_count as u32,
                self.max_entity_count,
            );

            for _ in 0..(self.max_entity_count - num_selected_children) {
                let (brain, body) = spawn_entity(
                    Brain::new(
                        self.neuron_hidden_layer_width,
                        self.neuron_output_fire_threshold,
                    ),
                    &mut used_positions,
                    self.grid_width,
                    self.grid_height,
                    &self.render_color_gradient,
                );

                next_generation.push((brain, body));
            }

            let max_selected = self.max_entity_count / self.entity_child_count as u32;

            let selected = selected.iter().take(max_selected as usize);

            // Spawn children of selected entities.
            for (brain, body) in selected {
                for _ in 0..self.entity_child_count {
                    let (x, y) =
                        get_random_position(&used_positions, self.grid_width, self.grid_height);

                    let brain = brain.clone();

                    // TODO: apply both structural and weight mutation.

                    // let dot = neural_net_to_dot(&brain);
                    // println!("{}", dot);

                    let child = (brain, Body::new(x, y, body.color()));
                    next_generation.push(child);
                }
            }

            self.entities = next_generation;
            self.sim_generation_number += 1;
            self.sim_current_step = 0;
        } else {
            self.sim_current_step += 1;
        }
    }

    // This whole render context dependency injection thing may not be what we want. It might be better to just
    // save state in the automata and have the render function access it directly.
    fn before_render(&self) -> RenderContext {
        let entity_colors = get_entity_colors(&self.entities, self.grid_width);

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

fn get_entity_colors(entities: &Vec<(Brain, Body)>, grid_width: u32) -> HashMap<usize, [u8; 4]> {
    let mut entity_colors: HashMap<usize, [u8; 4]> = HashMap::new();

    for (_brain, body) in entities {
        if !body.is_alive() {
            continue;
        }

        entity_colors.insert(
            grid_coords_to_index(body.x(), body.y(), grid_width),
            body.color(),
        );
    }

    entity_colors
}

fn spawn_entity(
    brain: Brain,
    occupied_positions: &mut Vec<usize>,
    grid_width: u32,
    grid_height: u32,
    gradient: &Gradient,
) -> (Brain, Body) {
    let (x, y) = get_random_position(occupied_positions, grid_width, grid_height);

    let color_index = rand::random::<f64>();
    let color = gradient.at(color_index).to_rgba8();

    let body = Body::new(x, y, color);

    (brain, body)
}

fn get_random_position(
    occupied_positions: &Vec<usize>,
    grid_width: u32,
    grid_height: u32,
) -> (u32, u32) {
    loop {
        let x = rand::random::<u32>() % grid_width;
        let y = rand::random::<u32>() % grid_height;

        let index = grid_coords_to_index(x, y, grid_width);

        if !occupied_positions.contains(&index) {
            return (x, y);
        }
    }
}
