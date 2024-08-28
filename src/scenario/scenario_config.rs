use crate::scenario::kill_zone::KillZone;

pub struct ScenarioConfig {
    pub generation_step_count: usize,
    pub kill_zones: Vec<KillZone>,
}
