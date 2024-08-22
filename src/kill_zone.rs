pub struct KillZone {
    pub start_time: f32,
    pub end_time: f32,
    pub position: (u32, u32),
    pub width: u32,
    pub height: u32,
}

pub fn is_point_in_killzone(
    kill_zones: &Vec<KillZone>,
    (x, y): (u32, u32),
    generation_time: f32,
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
