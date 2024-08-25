use std::collections::HashMap;

use cellular_automata::{
    automata::Automata,
    grid::grid_coords_to_index,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

use crate::settings::Settings;
use crate::{body::Body, kill_zone::distance_to_killzone};
use crate::{
    kill_zone::{is_point_in_killzone, KillZone},
    util::dot::clear_dot_files,
};
use crate::{neural_network::brain::Brain, util::dot::write_dot_file};
use colorgrad::Gradient;

pub struct LifeSim {
    entity_start_count: u32,
    entity_child_count: usize,
    entity_mutation_rate: f32,
    entity_mutation_magnitue: f32,

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

        let entity_start_count = settings.entity_start_count();

        let render_color_gradient = colorgrad::rainbow();

        let mut entities = Vec::new();
        let mut used_positions = Vec::<usize>::new();

        clear_dot_files();

        for i in 0..entity_start_count {
            let (brain, body) = spawn_entity(
                Brain::new(neuron_hidden_layer_width, neuron_output_fire_threshold),
                &mut used_positions,
                grid_width,
                grid_height,
            );

            write_dot_file(&brain, i);

            entities.push((brain, body));
        }

        let kill_zones = vec![
            KillZone {
                start_time: 30,
                end_time: 60,
                position: (120, 0),
                width: 30,
                height: grid_height,
            },
            KillZone {
                start_time: 60,
                end_time: 90,
                position: (90, 0),
                width: 30,
                height: grid_height,
            },
            KillZone {
                start_time: 90,
                end_time: 120,
                position: (60, 0),
                width: 30,
                height: grid_height,
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
                height: grid_height,
            },
        ];

        let sim_generation_steps = kill_zones.iter().map(|kz| kz.end_time).max().unwrap();

        Self {
            entity_child_count: settings.entity_child_count(),
            entity_start_count,
            entity_mutation_rate: settings.entity_mutation_rate(),
            entity_mutation_magnitue: settings.entity_mutation_magnitude(),
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
            sim_generation_steps,

            neuron_hidden_layer_width,
            neuron_output_fire_threshold,
        }
    }
}

type RenderContext = (HashMap<(u32, u32), f64>, Vec<KillZone>);

impl Automata<RenderContext> for LifeSim {
    fn update(&mut self) {
        let generation_time = self.sim_current_step as f32 / self.sim_generation_steps as f32;

        let active_kill_zones = self
            .kill_zones
            .iter()
            .filter(|kz| {
                self.sim_current_step >= kz.start_time && self.sim_current_step <= kz.end_time
            })
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
                self.entity_start_count,
            );

            for _ in 0..(self.entity_start_count - num_selected_children) {
                let (brain, body) = spawn_entity(
                    Brain::new(
                        self.neuron_hidden_layer_width,
                        self.neuron_output_fire_threshold,
                    ),
                    &mut used_positions,
                    self.grid_width,
                    self.grid_height,
                );

                next_generation.push((brain, body));
            }

            let max_selected = self.entity_start_count / self.entity_child_count as u32;

            let selected = selected.iter().take(max_selected as usize);

            clear_dot_files();

            // Spawn children of selected entities.
            for (brain, body) in selected {
                for i in 0..self.entity_child_count {
                    let (x, y) =
                        get_random_position(&used_positions, self.grid_width, self.grid_height);

                    let mut brain = brain.clone();
                    let mut body = Body::new(x, y, body.color_gradient_index());

                    if (i + 1) as f32 / self.entity_child_count as f32 <= self.entity_mutation_rate
                    {
                        let mutation_amount = (rand::random::<f64>() - 0.5)
                            * 2.0
                            * self.entity_mutation_magnitue as f64;

                        brain.mutate_connection(mutation_amount as f32);
                        body.mutate_color(mutation_amount);
                    }

                    write_dot_file(&brain, i as u32);

                    let child = (brain, body);
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
        let active_kill_zones = self
            .kill_zones
            .clone()
            .into_iter()
            .filter(|kz| {
                self.sim_current_step >= kz.start_time && self.sim_current_step <= kz.end_time
            })
            .collect::<Vec<KillZone>>();

        let entity_colors: HashMap<(u32, u32), f64> = self
            .entities
            .iter()
            .filter(|(_, body)| body.is_alive())
            .map(|(_, body)| ((body.x(), body.y()), body.color_gradient_index()))
            .collect();

        (entity_colors, active_kill_zones)
    }

    fn render(&self, context: &RenderContext, i: usize, pixel: &mut [u8]) {
        let (entity_colors, active_killzones) = context;
        let (vx, vy) =
            viewport_index_to_coords(i, self.render_viewport_width, self.render_viewport_height);
        let (x, y) = viewport_to_grid(vx, vy, self.render_pixel_scale);

        let color: [u8; 4] = if entity_colors.contains_key(&(x, y)) {
            self.render_color_gradient
                .at(entity_colors[&(x, y)])
                .to_rgba8()
        } else if is_point_in_killzone(active_killzones, (x, y), self.sim_current_step) {
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

fn spawn_entity(
    brain: Brain,
    occupied_positions: &mut Vec<usize>,
    grid_width: u32,
    grid_height: u32,
) -> (Brain, Body) {
    let (x, y) = get_random_position(occupied_positions, grid_width, grid_height);

    let body = Body::new(x, y, rand::random::<f64>());

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
