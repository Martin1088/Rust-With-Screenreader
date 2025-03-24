use rand::Rng;

fn generate_random_dna_sequence(length: usize) -> String {
    let bases = ['A', 'T', 'C', 'G'];
    let mut rng = rand::rng();
    (0..length).map(|_| bases[rng.random_range(0..4)]).collect()
}

pub fn generate_large_dna_example(count: usize, length: usize) -> Vec<String> {
    (0..count)
        .map(|_| generate_random_dna_sequence(length))
        .collect()
}
