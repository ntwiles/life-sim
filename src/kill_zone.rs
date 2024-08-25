#[derive(Debug, Clone)]
pub struct KillZone {
    pub start_time: usize,
    pub end_time: usize,
    pub position: (u32, u32),
    pub width: u32,
    pub height: u32,
}

pub fn distance_to_killzone(active_kill_zones: &Vec<&KillZone>, (x, y): (u32, u32)) -> (u32, u32) {
    active_kill_zones
        .iter()
        .map(|kill_zone| {
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

pub fn is_point_in_killzone(
    kill_zones: &Vec<KillZone>,
    (x, y): (u32, u32),
    generation_time: usize,
) -> bool {
    kill_zones.iter().any(|kz| {
        generation_time >= kz.start_time
            && generation_time <= kz.end_time
            && x >= kz.position.0
            && x < kz.position.0 + kz.width
            && y >= kz.position.1
            && y < kz.position.1 + kz.height
    })
}
