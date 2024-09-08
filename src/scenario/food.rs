use cellular_automata::grid::grid_coords_to_index;
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub fn generate_food(
    grid_width: usize,
    grid_height: usize,
    starting_food: usize,
) -> (Vec<bool>, Vec<(u32, u32)>) {
    let grid_size = grid_width * grid_height;

    let mut food_map = Vec::with_capacity(grid_size);
    let mut food_positions = Vec::with_capacity(starting_food);

    let mut rng = rand::thread_rng();

    for _ in 0..grid_size {
        food_map.push(false);
    }

    // Random distribution
    // for _ in 0..starting_food {
    //     let mut idx = rand::random::<usize>() % grid_size;

    //     while food_map[idx] {
    //         idx = rand::random::<usize>() % grid_size;
    //     }

    //     let pos = grid_index_to_coords(idx, grid_width as u32, grid_height as u32);

    //     food_map[idx] = true;
    //     food_positions.push(pos);
    // }

    // Clustered distribution
    let noise_scale = 0.05;
    // TODO: Why does threshold cause crashes below .5?
    let threshold = 0.3;

    let seed: u32 = rng.gen();
    let perlin = Perlin::new(seed);

    for y in 0..grid_height {
        for x in 0..grid_width {
            let idx = grid_coords_to_index(x as u32, y as u32, grid_width as u32);
            let noise_value = perlin.get([x as f64 * noise_scale, y as f64 * noise_scale, 0.0]);
            if noise_value > threshold {
                food_map[idx] = true;
                food_positions.push((x as u32, y as u32));
            }
        }
    }

    (food_map, food_positions)
}
