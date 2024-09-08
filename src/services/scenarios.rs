use serde::Deserialize;
use serde_json::{self, Error};

use crate::scenario::kill_zone::KillZone;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScenarioFile {
    pub kill_zones: Vec<KillZone>,

    pub supplement_entities: bool,
    pub limit_population: bool,

    pub starting_food: u32,
}

pub fn load_scenario(scenario_name: &str) -> Result<ScenarioFile, Error> {
    let file_path = format!("./data/scenarios/{}.json", scenario_name);

    let file = std::fs::File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);

    let scenario = serde_json::from_reader(reader)?;

    Ok(scenario)
}
