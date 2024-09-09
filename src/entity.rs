use cellular_automata::grid::grid_coords_to_index;

use crate::{
    body::Body,
    entity_config::EntityConfig,
    genome::{mutation::mutate_genome, random_genome},
    grid_config::GridConfig,
    neural_network::brain::Brain,
    neural_network_config::NeuralNetworkConfig,
};

#[derive(Debug)]
pub struct Entity {
    pub brain: Brain,
    pub body: Body,
    pub times_eaten: u32,
    pub times_irradiated: u32,
}

fn spawn_entity(
    brain: Brain,
    occupied_positions: &mut Vec<usize>,
    grid_config: &GridConfig,
) -> Entity {
    let (x, y) = get_random_position(occupied_positions, grid_config.width, grid_config.height);
    let body = Body::new(x, y, rand::random::<f64>());

    Entity {
        brain,
        body,
        times_eaten: 0,
        times_irradiated: 0,
    }
}

type SpawnedEntities = (Vec<Entity>, Vec<usize>);

pub fn spawn_entities(
    grid_config: &GridConfig,
    network_config: &NeuralNetworkConfig,
    num_entities: u32,
    existing_entities: Option<SpawnedEntities>,
) -> SpawnedEntities {
    let (mut entities, mut used_positions) = existing_entities.unwrap_or((Vec::new(), Vec::new()));

    for _ in 0..num_entities {
        let genome = random_genome(network_config);
        let entity = spawn_entity(Brain::from_genome(genome), &mut used_positions, grid_config);

        entities.push(entity);
    }

    (entities, used_positions)
}

pub fn spawn_next_generation(
    grid_config: &GridConfig,
    entity_config: &EntityConfig,
    network_config: &NeuralNetworkConfig,
    supplement_population: bool,
    limit_population: bool,
    mut selected: Vec<&Entity>,
) -> Vec<Entity> {
    if limit_population {
        let max_population = (entity_config.start_count / entity_config.child_count) as usize;
        selected.sort_by(|a, b| b.times_eaten.cmp(&a.times_eaten));
        selected.truncate(max_population);
    }

    let mut next_generation = Vec::<Entity>::new();
    let mut used_positions = Vec::<usize>::new();

    // Create children for each selected entity.
    for Entity { brain, .. } in selected {
        for _ in 0..entity_config.child_count {
            let mut genome = brain.genome.clone();

            let roll = rand::random::<f32>();

            if roll < network_config.mutation_rate {
                mutate_genome(&mut genome);
            }

            let brain = Brain::from_genome(genome);

            let entity = spawn_entity(brain, &mut used_positions, grid_config);
            next_generation.push(entity);
        }
    }

    // Generate new entities to fill the remaining slots.
    let num_remaining = entity_config.start_count as i32 - next_generation.len() as i32;

    if !supplement_population || num_remaining <= 0 {
        return next_generation;
    }

    let (next_generation, _) = spawn_entities(
        grid_config,
        network_config,
        num_remaining as u32,
        Some((next_generation, used_positions)),
    );

    next_generation
}

fn get_random_position(
    occupied_positions: &Vec<usize>,
    grid_width: u32,
    grid_height: u32,
) -> (u32, u32) {
    loop {
        let x = rand::random::<u32>() % grid_width;
        let y = rand::random::<u32>() % grid_height;

        let index = grid_coords_to_index(x, y, grid_width);

        if !occupied_positions.contains(&index) {
            return (x, y);
        }
    }
}
