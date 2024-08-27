use cellular_automata::grid::grid_coords_to_index;

use crate::{
    body::Body, entity_config::EntityConfig, grid_config::GridConfig, neural_network::brain::Brain,
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
    num_entities: u32,
    neuron_layer_width: usize,
    existing_entities: Option<SpawnedEntities>,
) -> SpawnedEntities {
    let (mut entities, mut used_positions) = existing_entities.unwrap_or((Vec::new(), Vec::new()));

    for _ in 0..num_entities {
        let (brain, body) = spawn_entity(
            Brain::new(neuron_layer_width),
            &mut used_positions,
            grid_config,
        );

        entities.push((brain, body));
    }

    (entities, used_positions)
}

pub fn spawn_next_generation(
    grid_config: &GridConfig,
    entity_config: &EntityConfig,
    neuron_layer_width: usize,

    selected: Vec<&(Brain, Body)>,
) -> Vec<(Brain, Body)> {
    let mut next_generation = Vec::<(Brain, Body)>::new();
    let mut used_positions = Vec::<usize>::new();

    let max_selected = entity_config.start_count / entity_config.child_count;
    let selected = selected.iter().take(max_selected as usize);

    // Create children for each selected entity.
    for (brain, _) in selected {
        for _ in 0..entity_config.child_count {
            let (brain, body) = spawn_entity(brain.clone(), &mut used_positions, grid_config);

            next_generation.push((brain, body));
        }
    }

    // Apply mutations.
    let num_to_mutate =
        (next_generation.len() as f32 * entity_config.mutation_rate).floor() as usize;
    for (brain, body) in next_generation.iter_mut().take(num_to_mutate) {
        let mutation_amount =
            (rand::random::<f64>() - 0.5) * 2.0 * entity_config.mutation_magnitude as f64;

        // TODO: Apply structural mutations.
        brain.mutate_connection(mutation_amount as f32);
        body.mutate_color(mutation_amount);
    }

    // Generate new entities to fill the remaining slots.
    let num_remaining = entity_config.start_count - next_generation.len() as u32;
    let (next_generation, _) = spawn_entities(
        grid_config,
        num_remaining,
        neuron_layer_width,
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
