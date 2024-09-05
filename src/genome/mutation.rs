use rand::Rng;

use super::gene::Gene;

pub fn mutate_genome(genome: &mut Vec<Gene>) {
    let mut rng = rand::thread_rng();

    let element_index = rng.gen_range(0..genome.len());
    let bit_index16 = 1 << rng.gen_range(0..16);

    let chance: f32 = rng.gen();
    let gene = &mut genome[element_index];

    if chance < 0.05 {
        gene.source_discriminant ^= bit_index16;
    } else if chance < 0.1 {
        gene.source_instance ^= bit_index16;
    } else if chance < 0.15 {
        gene.target_discriminant ^= bit_index16;
    } else if chance < 0.2 {
        gene.target_instance ^= bit_index16;
    } else {
        let weight_mutation = rng.gen_range(-0.1..0.1);
        gene.weight += weight_mutation;
    }
}
