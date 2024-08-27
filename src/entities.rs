use cellular_automata::grid::grid_coords_to_index;

use crate::{body::Body, neural_network::brain::Brain};

// TODO: These functions have been refactored multiple times; check that the logic still all makes sense.

fn spawn_entity(
    brain: Brain,
    occupied_positions: &mut Vec<usize>,
    grid_width: u32,
    grid_height: u32,
) -> (Brain, Body) {
    let (x, y) = get_random_position(occupied_positions, grid_width, grid_height);
    let body = Body::new(x, y, rand::random::<f64>());
    (brain, body)
}

type SpawnedEntities = (Vec<(Brain, Body)>, Vec<usize>);

pub fn spawn_entities(
    grid_width: u32,
    grid_height: u32,
    num_entities: u32,
    neuron_layer_width: usize,
    existing_entities: Option<SpawnedEntities>,
) -> SpawnedEntities {
    let (mut entities, mut used_positions) = existing_entities.unwrap_or((Vec::new(), Vec::new()));

    for _ in 0..num_entities {
        let (brain, body) = spawn_entity(
            Brain::new(neuron_layer_width),
            &mut used_positions,
            grid_width,
            grid_height,
        );

        entities.push((brain, body));
    }

    (entities, used_positions)
}

pub fn spawn_next_generation(
    grid_width: u32,
    grid_height: u32,
    max_entities: u32,
    neuron_layer_width: usize,
    entity_child_count: u32,
    entity_mutation_rate: f32,
    entity_mutation_magnitude: f32,
    selected: Vec<&(Brain, Body)>,
) -> Vec<(Brain, Body)> {
    let mut next_generation = Vec::<(Brain, Body)>::new();
    let mut used_positions = Vec::<usize>::new();

    let max_selected = max_entities / entity_child_count;
    let selected = selected.iter().take(max_selected as usize);

    for (brain, _) in selected {
        for _ in 0..entity_child_count {
            let (brain, body) =
                spawn_entity(brain.clone(), &mut used_positions, grid_width, grid_height);

            next_generation.push((brain, body));
        }
    }

    // Apply mutations.
    let num_to_mutate = (next_generation.len() as f32 * entity_mutation_rate).floor() as usize;

    for (brain, body) in next_generation.iter_mut().take(num_to_mutate) {
        let mutation_amount =
            (rand::random::<f64>() - 0.5) * 2.0 * entity_mutation_magnitude as f64;

        brain.mutate_connection(mutation_amount as f32);
        body.mutate_color(mutation_amount);
    }

    let num_remaining = max_entities - next_generation.len() as u32;

    let (next_generation, _) = spawn_entities(
        grid_width,
        grid_height,
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
