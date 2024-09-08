use cellular_automata::grid::grid_coords_to_index;

use super::{food::generate_food, kill_zone::KillZone};

pub struct Scenario {
    pub generation_step_count: usize,

    pub starting_kill_zones: Vec<KillZone>,
    pub remaining_kill_zones: Vec<usize>,
    pub active_kill_zones: Vec<usize>,

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

    pub fn shortest_killzone_displacement(&self, (x, y): (u32, u32)) -> (u32, u32) {
        // TODO: This has a map and a fold and can probably be optimized.
        self.active_kill_zones
            .iter()
            .map(|i| {
                let kill_zone = &self.starting_kill_zones[*i];
                let (kx, ky) = kill_zone.position;
                let (kz_width, kz_height) = (kill_zone.width, kill_zone.height);

                let dist_x = if x < kx {
                    kx - x
                } else if x >= kx + kz_width {
                    x - (kx + kz_width - 1)
                } else {
                    0
                };

                let dist_y = if y < ky {
                    ky - y
                } else if y >= ky + kz_height {
                    y - (ky + kz_height - 1)
                } else {
                    0
                };

                (dist_x, dist_y)
            })
            .fold((u32::MAX, u32::MAX), |(min_dx, min_dy), (dx, dy)| {
                (min_dx.min(dx), min_dy.min(dy))
            })
    }

    pub fn shortest_food_displacement(&self, (x, y): (u32, u32)) -> (u32, u32) {
        let mut min_dx = u32::MAX;
        let mut min_dy = u32::MAX;

        for (fx, fy) in &self.food_positions {
            let dx = if x < *fx { fx - x } else { x - fx };
            let dy = if y < *fy { fy - y } else { y - fy };

            if dx < min_dx {
                min_dx = dx;
            }

            if dy < min_dy {
                min_dy = dy;
            }
        }

        (min_dx, min_dy)
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
