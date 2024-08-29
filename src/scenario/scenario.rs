use super::kill_zone::KillZone;

pub struct Scenario {
    pub generation_step_count: usize,

    pub starting_kill_zones: Vec<KillZone>,
    pub remaining_kill_zones: Vec<usize>,
    pub active_kill_zones: Vec<usize>,
}

impl Scenario {
    pub fn new(starting_kill_zones: Vec<KillZone>) -> Self {
        let generation_step_count = starting_kill_zones
            .iter()
            .map(|kz| kz.end_time)
            .max()
            .unwrap();

        Self {
            generation_step_count,
            remaining_kill_zones: (0..starting_kill_zones.len()).collect(),
            starting_kill_zones,
            active_kill_zones: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.remaining_kill_zones = (0..self.starting_kill_zones.len()).collect();
        self.active_kill_zones = Vec::new();
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
