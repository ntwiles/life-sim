pub fn generate_food(world_size: usize, starting_food: u32) -> Vec<bool> {
    let mut food = Vec::with_capacity(world_size);

    for _ in 0..world_size {
        food.push(false);
    }

    for _ in 0..starting_food {
        let mut idx = rand::random::<usize>() % world_size;

        while food[idx] {
            idx = rand::random::<usize>() % world_size;
        }

        food[idx] = true;
    }

    food
}
