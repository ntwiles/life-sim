use rand::Rng;

use super::gene::Gene;

pub fn flip_random_bit(genome: &mut Vec<Gene>) {
    let mut rng = rand::thread_rng();

    let element_index = rng.gen_range(0..genome.len());
    let bit_index16 = 1 << rng.gen_range(0..16);

    let chance: f32 = rng.gen(); // Generates a random float between 0.0 and 1.0

    if chance < 0.33 {
        genome[element_index].source_discriminant ^= bit_index16;
    } else if chance < 0.66 {
        genome[element_index].target_discriminant ^= bit_index16;
    } else {
        // TODO: mutate weight.
    }
}
