use std::collections::HashMap;

use cellular_automata::{
    automata::Automata,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

use crate::kill_zone::{is_point_in_killzone, KillZone};
use crate::neural_network::brain::Brain;
use crate::settings::Settings;
use crate::{
    body::Body,
    entities::{spawn_entities, spawn_next_generation},
    kill_zone::distance_to_killzone,
};
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

    sim_current_step: usize,
    sim_generation_steps: usize,
    sim_generation_number: u32,
}

impl LifeSim {
    pub fn new(settings: &Settings) -> Self {
        let grid_width = settings.grid_width();
        let grid_height = settings.grid_height();

        let neuron_hidden_layer_width = settings.neuron_hidden_layer_width();

        let entity_start_count = settings.entity_start_count();

        let render_color_gradient = colorgrad::rainbow();

        let (entities, _) = spawn_entities(
            grid_width,
            grid_height,
            entity_start_count,
            neuron_hidden_layer_width,
            None,
        );

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

                let decision = brain.decide(generation_time, danger_dist, grid_size);
                body.update(decision, grid_size);
            }
        }

        if self.sim_current_step > self.sim_generation_steps {
            let selected: Vec<&(Brain, Body)> = self
                .entities
                .iter()
                .filter(|(_, body)| body.is_alive())
                .collect();

            println!(
                "Generation {} over. Survivors {}/{} ({:.2}%)",
                self.sim_generation_number,
                selected.len(),
                self.entities.len(),
                selected.len() as f32 / self.entities.len() as f32 * 100.0
            );

            let next_generation = spawn_next_generation(
                self.grid_width,
                self.grid_height,
                self.entity_start_count,
                self.neuron_hidden_layer_width,
                self.entity_child_count as u32,
                self.entity_mutation_rate,
                self.entity_mutation_magnitue,
                selected,
            );

            println!("Next generation size: {}", next_generation.len());

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
