use crate::kill_zone::KillZone;

pub struct SimulationConfig {
    pub generation_step_count: usize,
    pub kill_zones: Vec<KillZone>,
}
