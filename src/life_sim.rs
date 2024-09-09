use std::collections::HashMap;

use cellular_automata::{
    automata::Automata,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

use crate::entity_config::EntityConfig;
use crate::{
    entity::{spawn_entities, spawn_next_generation, Entity},
    neural_network_config::NeuralNetworkConfig,
    rendering::additive_blend,
    services::dot::write_dot_file,
};
use crate::{grid_config::GridConfig, scenario::scenario::Scenario};
use crate::{render_config::RenderConfig, vector_2d::Vector2D};

pub struct LifeSim {
    entities: Vec<Entity>,
    sim_current_step: usize,
    sim_generation_number: u32,

    scenario: Scenario,

    grid_config: GridConfig,
    render_config: RenderConfig,
    entity_config: EntityConfig,
    network_config: NeuralNetworkConfig,
}

impl LifeSim {
    pub fn new(
        scenario: Scenario,
        grid_config: GridConfig,
        render_config: RenderConfig,
        entity_config: EntityConfig,
        network_config: NeuralNetworkConfig,
    ) -> Self {
        let (entities, _) = spawn_entities(
            &grid_config,
            &network_config,
            entity_config.start_count,
            None,
        );

        for i in 0..4 {
            let Entity { brain, .. } = &entities[i];
            write_dot_file(&brain, i);
        }

        Self {
            scenario,

            entity_config,
            grid_config,
            render_config,
            network_config,

            entities,
            sim_current_step: 0,
            sim_generation_number: 0,
        }
    }

    fn selection_filter(entity: &&Entity, scenario: &Scenario) -> bool {
        if let Some(food) = &scenario.food {
            let food_selection = if food.cull_for_starvation {
                entity.times_eaten > 0
            } else {
                true
            };

            entity.body.is_alive && food_selection
        } else {
            entity.body.is_alive
        }
    }

    fn start_new_generation(&mut self) {
        let selected: Vec<&Entity> = self
            .entities
            .iter()
            .filter(|e| Self::selection_filter(e, &self.scenario))
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
            self.scenario.supplement_population,
            self.scenario.limit_population,
            selected,
        );

        for (i, entity) in next_generation.iter().enumerate().take(4) {
            write_dot_file(&entity.brain, i);
        }

        self.entities = next_generation;
        self.sim_generation_number += 1;
        self.sim_current_step = 0;
    }
}

type EntityColors = HashMap<(u32, u32), f64>;

impl Automata<EntityColors> for LifeSim {
    fn update(&mut self) {
        let generation_time =
            self.sim_current_step as f32 / self.scenario.generation_step_count as f32;

        self.scenario.update(self.sim_current_step);

        for entity in &mut self.entities {
            if !entity.body.is_alive {
                continue;
            }

            if self
                .scenario
                .is_point_in_kill_zone((entity.body.x, entity.body.y), self.sim_current_step)
            {
                entity.body.is_alive = false;
                continue;
            }

            let (killzone_dist, killzone_disp) = self
                .scenario
                .shortest_killzone_displacement((entity.body.x, entity.body.y));

            let killzone_dir = killzone_disp.normalize();
            let danger_angle = killzone_dir.y.atan2(killzone_dir.x);

            let mut food_angle: f32 = 0.0;

            if self.scenario.food.is_some() {
                let (_, food_disp) = self
                    .scenario
                    .shortest_food_displacement((entity.body.x, entity.body.y));

                let food_dist_xy = Vector2D {
                    x: food_disp.0 as f32,
                    y: food_disp.1 as f32,
                };

                let food_dir = food_dist_xy.normalize();
                food_angle = food_dir.y.atan2(food_dir.x);
            }

            let pos = (entity.body.x as u32, entity.body.y as u32);

            if self.scenario.food.is_some() {
                if self.scenario.is_food_at_point(pos) {
                    self.scenario.consume_food_at_point(pos);
                    entity.times_eaten += 1;
                }
            }

            let decision = entity.brain.decide(
                generation_time,
                killzone_dist,
                danger_angle.sin(),
                danger_angle.cos(),
                food_angle.sin(),
                food_angle.cos(),
            );

            entity.body.update(decision, &self.grid_config);
        }

        if self.sim_current_step >= self.scenario.generation_step_count {
            self.scenario.reset();
            self.start_new_generation();
        } else {
            self.sim_current_step += 1;
        }
    }

    // This whole render context dependency injection thing may not be what we want. It might be better to just
    // save state in the automata and have the render function access it directly.
    fn before_render(&self) -> EntityColors {
        let mut entity_colors = HashMap::new();

        for entity in &self.entities {
            if entity.body.is_alive {
                entity_colors.insert(
                    (entity.body.x as u32, entity.body.y as u32),
                    entity.body.color_gradient_index,
                );
            }
        }

        entity_colors
    }

    fn render(&self, entity_colors: &EntityColors, i: usize, pixel: &mut [u8]) {
        let (vx, vy) = viewport_index_to_coords(
            i,
            self.render_config.viewport_width,
            self.render_config.viewport_height,
        );

        let (x, y) = viewport_to_grid(vx, vy, self.render_config.pixel_scale);

        let color: [u8; 4] = if entity_colors.contains_key(&(x, y)) {
            [0, 140, 200, 255]
        } else if self.scenario.food.is_some() && self.scenario.is_food_at_point((x, y)) {
            [20, 200, 0, 255]
        } else {
            self.render_config.background_color
        };

        let color = if self
            .scenario
            .is_point_in_kill_zone((x, y), self.sim_current_step)
        {
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
