use cellular_automata::grid::grid_coords_to_index;
use serde::Deserialize;

use crate::{services::scenarios::ScenarioFile, vector_2d::Vector2D};

use super::{
    food::{generate_food, ScenarioFood},
    radiation_zone::ScenarioRadiation,
};

#[derive(Deserialize)]
pub struct Scenario {
    pub generation_step_count: usize,

    pub supplement_population: bool,
    pub limit_population: bool,

    pub radiation: Option<ScenarioRadiation>,

    pub food: Option<ScenarioFood>,

    grid_width: u32,
    grid_height: u32,
}

impl Scenario {
    pub fn from_file(config: ScenarioFile, grid_width: u32, grid_height: u32) -> Self {
        let mut generation_step_count = 0;

        let radiation = if let Some(radiation_config) = config.radiation {
            let starting_rad_zones = radiation_config.zones;

            generation_step_count = starting_rad_zones
                .iter()
                .map(|kz| kz.end_time)
                .max()
                .unwrap();

            Some(ScenarioRadiation {
                death_threshold: radiation_config.death_threshold,
                remaining_rad_zones: (0..starting_rad_zones.len()).collect(),
                starting_rad_zones,
                active_rad_zones: Vec::new(),
            })
        } else {
            None
        };

        let food = if let Some(food_config) = config.food {
            let (food_map, food_positions) = generate_food(
                grid_width as usize,
                grid_height as usize,
                food_config.starting_food as usize,
            );

            Some(ScenarioFood {
                starting_food: food_config.starting_food,
                cull_for_starvation: food_config.cull_for_starvation,
                food_map,
                food_positions,
            })
        } else {
            None
        };

        Self {
            generation_step_count,
            radiation,
            food,
            grid_width,
            grid_height,
            supplement_population: config.supplement_population,
            limit_population: config.limit_population,
        }
    }

    pub fn reset(&mut self) {
        if let Some(radiation) = &mut self.radiation {
            radiation.remaining_rad_zones = (0..radiation.starting_rad_zones.len()).collect();
            radiation.active_rad_zones = Vec::new();
        }

        if let Some(food) = &mut self.food {
            let (food_map, food_positions) = generate_food(
                self.grid_width as usize,
                self.grid_height as usize,
                food.starting_food as usize,
            );

            food.food_map = food_map;
            food.food_positions = food_positions;
        }
    }

    pub fn update(&mut self, current_step: usize) {
        // TODO: We can know how many steps away the next need for an update is. We can skip
        // updating until that point.

        if let Some(radiation) = &mut self.radiation {
            radiation
                .remaining_rad_zones
                .retain(|kzi| current_step < radiation.starting_rad_zones[*kzi].end_time);

            radiation.active_rad_zones =
                radiation
                    .remaining_rad_zones
                    .iter()
                    .fold(Vec::new(), |mut acc, kzi| {
                        let kz = &radiation.starting_rad_zones[*kzi];
                        if current_step >= kz.start_time && current_step <= kz.end_time {
                            acc.push(*kzi);
                        }

                        acc
                    });
        }
    }

    pub fn shortest_rad_zone_displacement(&self, (x, y): (u32, u32)) -> (f32, Vector2D) {
        let mut min_dist = f32::MAX;
        let mut min_disp = Vector2D { x: 0.0, y: 0.0 };

        let radiation = self.radiation.as_ref().unwrap();

        for i in &radiation.active_rad_zones {
            let kz = &radiation.starting_rad_zones[*i];

            let dx: i32 = if x < kz.position.0 {
                kz.position.0 as i32 - x as i32
            } else if x >= kz.position.0 + kz.width {
                (kz.position.0 + kz.width) as i32 - x as i32
            } else {
                0
            };

            let dy: i32 = if y < kz.position.1 {
                kz.position.1 as i32 - y as i32
            } else if y >= kz.position.1 + kz.height {
                (kz.position.1 + kz.height) as i32 - y as i32
            } else {
                0
            };

            let disp = Vector2D {
                x: dx as f32,
                y: dy as f32,
            };

            let dist = disp.magnitude();

            if dist < min_dist {
                min_dist = dist;
                min_disp = disp;
            }
        }

        (min_dist, min_disp)
    }

    pub fn shortest_food_displacement(&self, (x, y): (u32, u32)) -> (f32, (i32, i32)) {
        let mut min_dist = f32::MAX;
        let mut min_disp = (i32::MAX, i32::MAX);

        let food = self.food.as_ref().unwrap();

        for (fx, fy) in &food.food_positions {
            let dx = *fx as i32 - x as i32;
            let dy = *fy as i32 - y as i32;

            let vec = Vector2D {
                x: *fx as f32 - x as f32,
                y: *fy as f32 - y as f32,
            };

            let dist = vec.magnitude();

            if dist < min_dist {
                min_dist = dist;
                min_disp = (dx, dy);
            }
        }

        (min_dist, min_disp)
    }

    pub fn is_food_at_point(&self, (x, y): (u32, u32)) -> bool {
        let index = grid_coords_to_index(x, y, self.grid_width);
        self.food.as_ref().unwrap().food_map[index]
    }

    pub fn consume_food_at_point(&mut self, (x, y): (u32, u32)) {
        let index = grid_coords_to_index(x, y, self.grid_width);
        let food = self.food.as_mut().unwrap();
        food.food_map[index] = false;
        food.food_positions.retain(|&(fx, fy)| fx != x || fy != y);
    }

    pub fn is_point_in_rad_zone(&self, (x, y): (u32, u32), generation_time: usize) -> bool {
        let radiation = self.radiation.as_ref().unwrap();

        radiation.active_rad_zones.iter().any(|i| {
            let kz = &radiation.starting_rad_zones[*i];
            generation_time >= kz.start_time
                && generation_time <= kz.end_time
                && x >= kz.position.0
                && x < kz.position.0 + kz.width
                && y >= kz.position.1
                && y < kz.position.1 + kz.height
        })
    }
}
