use cellular_automata::grid::grid_coords_to_index;

use crate::vector_2d::Vector2D;

use super::{food::generate_food, kill_zone::KillZone};

pub struct Scenario {
    pub generation_step_count: usize,

    pub starting_kill_zones: Vec<KillZone>,
    pub remaining_kill_zones: Vec<usize>,
    pub active_kill_zones: Vec<usize>,

    pub supplement_entities: bool,
    pub limit_population: bool,

    starting_food: u32,
    food_map: Vec<bool>,
    food_positions: Vec<(u32, u32)>,

    grid_width: u32,
    grid_height: u32,
}

impl Scenario {
    pub fn new(
        starting_kill_zones: Vec<KillZone>,
        starting_food: u32,
        grid_width: u32,
        grid_height: u32,
        supplement_entities: bool,
        limit_population: bool,
    ) -> Self {
        let generation_step_count = starting_kill_zones
            .iter()
            .map(|kz| kz.end_time)
            .max()
            .unwrap();

        let (food_map, food_positions) = generate_food(
            grid_width as usize,
            grid_height as usize,
            starting_food as usize,
        );

        Self {
            generation_step_count,
            remaining_kill_zones: (0..starting_kill_zones.len()).collect(),
            starting_kill_zones,
            active_kill_zones: Vec::new(),
            food_map,
            food_positions,
            starting_food,
            grid_width,
            grid_height,
            supplement_entities,
            limit_population,
        }
    }

    pub fn reset(&mut self) {
        self.remaining_kill_zones = (0..self.starting_kill_zones.len()).collect();
        self.active_kill_zones = Vec::new();

        let (food_map, food_positions) = generate_food(
            self.grid_width as usize,
            self.grid_height as usize,
            self.starting_food as usize,
        );

        self.food_map = food_map;
        self.food_positions = food_positions;
    }

    pub fn update(&mut self, current_step: usize) {
        // TODO: We can know how many steps away the next need for an update is. We can skip
        // updating until that point.

        self.remaining_kill_zones
            .retain(|kzi| current_step < self.starting_kill_zones[*kzi].end_time);

        self.active_kill_zones =
            self.remaining_kill_zones
                .iter()
                .fold(Vec::new(), |mut acc, kzi| {
                    let kz = &self.starting_kill_zones[*kzi];
                    if current_step >= kz.start_time && current_step <= kz.end_time {
                        acc.push(*kzi);
                    }

                    acc
                });
    }

    pub fn shortest_killzone_displacement(&self, (x, y): (u32, u32)) -> (f32, Vector2D) {
        let mut min_dist = f32::MAX;
        let mut min_disp = Vector2D { x: 0.0, y: 0.0 };

        for i in &self.active_kill_zones {
            let kz = &self.starting_kill_zones[*i];

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

        for (fx, fy) in &self.food_positions {
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
        self.food_map[index]
    }

    pub fn consume_food_at_point(&mut self, (x, y): (u32, u32)) {
        let index = grid_coords_to_index(x, y, self.grid_width);
        self.food_map[index] = false;
        self.food_positions.retain(|&(fx, fy)| fx != x || fy != y);
    }

    pub fn is_point_in_kill_zone(&self, (x, y): (u32, u32), generation_time: usize) -> bool {
        self.active_kill_zones.iter().any(|i| {
            let kz = &self.starting_kill_zones[*i];
            generation_time >= kz.start_time
                && generation_time <= kz.end_time
                && x >= kz.position.0
                && x < kz.position.0 + kz.width
                && y >= kz.position.1
                && y < kz.position.1 + kz.height
        })
    }
}
