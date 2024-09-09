use serde::Deserialize;
use serde_json::{self, Error};

use crate::scenario::radiation_zone::RadiationZone;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodFile {
    pub starting_food: u32,
    pub cull_for_starvation: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadiationFile {
    pub death_threshold: Option<u32>,
    pub zones: Vec<RadiationZone>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScenarioFile {
    pub supplement_population: bool,
    pub limit_population: bool,

    pub food: Option<FoodFile>,
    pub radiation: Option<RadiationFile>,
}

pub fn load_scenario(scenario_name: &str) -> Result<ScenarioFile, Error> {
    let file_path = format!("./data/scenarios/{}.json", scenario_name);

    let file = std::fs::File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);

    let scenario = serde_json::from_reader(reader)?;

    Ok(scenario)
}
