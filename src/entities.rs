use cellular_automata::grid::grid_coords_to_index;

use crate::{
    body::Body,
    entity_config::EntityConfig,
    genome::{mutation::mutate_genome, random_genome},
    grid_config::GridConfig,
    neural_network::brain::Brain,
    neural_network_config::NeuralNetworkConfig,
};

fn spawn_entity(
    brain: Brain,
    occupied_positions: &mut Vec<usize>,
    grid_config: &GridConfig,
) -> (Brain, Body) {
    let (x, y) = get_random_position(occupied_positions, grid_config.width, grid_config.height);
    let body = Body::new(x, y, rand::random::<f64>());
    (brain, body)
}

type SpawnedEntities = (Vec<(Brain, Body)>, Vec<usize>);

pub fn spawn_entities(
    grid_config: &GridConfig,
    network_config: &NeuralNetworkConfig,
    num_entities: u32,
    existing_entities: Option<SpawnedEntities>,
) -> SpawnedEntities {
    let (mut entities, mut used_positions) = existing_entities.unwrap_or((Vec::new(), Vec::new()));

    for _ in 0..num_entities {
        let genome = random_genome(network_config);
        let (brain, body) =
            spawn_entity(Brain::from_genome(genome), &mut used_positions, grid_config);

        entities.push((brain, body));
    }

    (entities, used_positions)
}

pub fn spawn_next_generation(
    grid_config: &GridConfig,
    entity_config: &EntityConfig,
    network_config: &NeuralNetworkConfig,

    selected: Vec<&(Brain, Body)>,
) -> Vec<(Brain, Body)> {
    let mut next_generation = Vec::<(Brain, Body)>::new();
    let mut used_positions = Vec::<usize>::new();

    let max_selected = entity_config.start_count / entity_config.child_count;
    let selected = selected.iter().take(max_selected as usize);

    // Create children for each selected entity.
    for (brain, _) in selected {
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
    let num_remaining = entity_config.start_count - next_generation.len() as u32;

    let (next_generation, _) = spawn_entities(
        grid_config,
        network_config,
        num_remaining,
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
