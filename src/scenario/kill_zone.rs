use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KillZone {
    pub start_time: usize,
    pub end_time: usize,
    pub position: (u32, u32),
    pub width: u32,
    pub height: u32,
}
