use std::collections::HashMap;

use cellular_automata::{
    automata::Automata,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

use crate::{
    body::Body,
    entities::{spawn_entities, spawn_next_generation},
    neural_network_config::NeuralNetworkConfig,
    rendering::additive_blend,
    scenario::kill_zone::distance_to_killzone,
};
use crate::{entity_config::EntityConfig, neural_network::brain::Brain};
use crate::{grid_config::GridConfig, scenario::scenario_config::ScenarioConfig};
use crate::{render_config::RenderConfig, scenario::kill_zone::is_point_in_kill_zone};

pub struct LifeSim {
    entities: Vec<(Brain, Body)>,
    sim_current_step: usize,
    sim_generation_number: u32,

    grid_config: GridConfig,
    render_config: RenderConfig,
    entity_config: EntityConfig,
    network_config: NeuralNetworkConfig,
    scenario_config: ScenarioConfig,

    active_kill_zones: Vec<usize>,
}

impl LifeSim {
    pub fn new(
        grid_config: GridConfig,
        render_config: RenderConfig,
        entity_config: EntityConfig,
        network_config: NeuralNetworkConfig,
        scenario_config: ScenarioConfig,
    ) -> Self {
        let (entities, _) = spawn_entities(
            &grid_config,
            entity_config.start_count,
            network_config.hidden_layer_width,
            None,
        );

        Self {
            entity_config,
            grid_config,
            render_config,
            network_config,
            scenario_config,

            entities,
            sim_current_step: 0,
            sim_generation_number: 0,
            active_kill_zones: Vec::new(),
        }
    }

    fn start_new_generation(&mut self) {
        let selected: Vec<&(Brain, Body)> = self
            .entities
            .iter()
            .filter(|(_, body)| body.is_alive)
            .collect();

        println!(
            "Generation {} over. Survivors {}/{} ({:.2}%)",
            self.sim_generation_number,
            selected.len(),
            self.entities.len(),
            selected.len() as f32 / self.entities.len() as f32 * 100.0
        );

        let next_generation = spawn_next_generation(
            &self.grid_config,
            &self.entity_config,
            &self.network_config,
            selected,
        );

        self.entities = next_generation;
        self.sim_generation_number += 1;
        self.sim_current_step = 0;
    }
}

type EntityColors = HashMap<(u32, u32), f64>;

impl Automata<EntityColors> for LifeSim {
    fn update(&mut self) {
        let generation_time =
            self.sim_current_step as f32 / self.scenario_config.generation_step_count as f32;

        self.active_kill_zones = self.scenario_config.kill_zones.iter().enumerate().fold(
            Vec::new(),
            |mut acc, (i, kz)| {
                if self.sim_current_step >= kz.start_time && self.sim_current_step <= kz.end_time {
                    acc.push(i);
                }
                acc
            },
        );

        for (brain, body) in &mut self.entities {
            if !body.is_alive {
                continue;
            }

            let danger_dist = distance_to_killzone(
                &self.scenario_config.kill_zones,
                &self.active_kill_zones,
                (body.x, body.y),
            );

            if danger_dist == (0, 0) {
                body.is_alive = false;
            } else {
                let decision = brain.decide(generation_time, danger_dist, &self.grid_config);
                body.update(decision, &self.grid_config);
            }
        }

        if self.sim_current_step >= self.scenario_config.generation_step_count {
            self.start_new_generation();
        } else {
            self.sim_current_step += 1;
        }
    }

    // This whole render context dependency injection thing may not be what we want. It might be better to just
    // save state in the automata and have the render function access it directly.
    fn before_render(&self) -> EntityColors {
        self.entities
            .iter()
            .filter(|(_, body)| body.is_alive)
            .map(|(_, body)| ((body.x, body.y), body.color_gradient_index))
            .collect()
    }

    fn render(&self, entity_colors: &EntityColors, i: usize, pixel: &mut [u8]) {
        let (vx, vy) = viewport_index_to_coords(
            i,
            self.render_config.viewport_width,
            self.render_config.viewport_height,
        );

        let (x, y) = viewport_to_grid(vx, vy, self.render_config.pixel_scale);

        let color: [u8; 4] = if entity_colors.contains_key(&(x, y)) {
            self.render_config
                .color_gradient
                .at(entity_colors[&(x, y)])
                .to_rgba8()
        } else {
            self.render_config.background_color
        };

        let color = if is_point_in_kill_zone(
            &self.scenario_config.kill_zones,
            &self.active_kill_zones,
            (x, y),
            self.sim_current_step,
        ) {
            additive_blend(self.render_config.killzone_color, color)
        } else {
            color
        };

        pixel.copy_from_slice(&color);
    }

    fn grid_width(&self) -> u32 {
        self.grid_config.width
    }

    fn grid_height(&self) -> u32 {
        self.grid_config.height
    }

    fn render_pixel_scale(&self) -> u32 {
        self.render_config.pixel_scale
    }
}
