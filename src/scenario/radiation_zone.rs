use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadiationZone {
    pub start_time: usize,
    pub end_time: usize,
    pub position: (u32, u32),
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize)]
pub struct ScenarioRadiation {
    pub death_threshold: Option<u32>,
    pub starting_rad_zones: Vec<RadiationZone>,
    pub remaining_rad_zones: Vec<usize>,
    pub active_rad_zones: Vec<usize>,
}
