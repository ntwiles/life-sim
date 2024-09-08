use cellular_automata::grid::grid_index_to_coords;

pub fn generate_food(
    grid_width: usize,
    grid_height: usize,
    starting_food: usize,
) -> (Vec<bool>, Vec<(u32, u32)>) {
    let grid_size = grid_width * grid_height;

    let mut food_map = Vec::with_capacity(grid_size);
    let mut food_positions = Vec::with_capacity(starting_food);

    for _ in 0..grid_size {
        food_map.push(false);
    }

    for _ in 0..starting_food {
        let mut idx = rand::random::<usize>() % grid_size;

        while food_map[idx] {
            idx = rand::random::<usize>() % grid_size;
        }

        let pos = grid_index_to_coords(idx, grid_width as u32, grid_height as u32);

        food_map[idx] = true;
        food_positions.push(pos);
    }

    (food_map, food_positions)
}
