use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use sequencing_dna::analyse::{
    bits_handler::EncodedDNA,
    check_seq::{get_balance, get_balance_as_bits},
};

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(500)
        .warm_up_time(std::time::Duration::from_millis(200))
        .measurement_time(std::time::Duration::from_secs(5))
}

pub fn dna_benchmark(c: &mut Criterion) {
    let example = vec![
        "GTCAGTCAAGCTAGTC",
        "AGTAGTACGCTAGCTA",
        "CTCTGACAGTCAAGTC",
        "TGCATGCACTGATGCA",
        "ATGCATGCAGTCATGC",
        "GATCTGACATGCAGTC",
        "ACGTACGTGATCGTAC",
        "GTAGCTAGTGACTGCA",
        "CTAGCTAGGACTAGCT",
        "AGCTAGCTGACTAGGC",
        "CGATCGATGATCTAGT",
        "TGCAATGCAGCTAGTC",
        "GCTAGTCAAGCTAGTC",
        "ATCGATCGACTAGCTA",
        "TGCAGCTAGTGCATAG",
        "AGCTGACTGACTAGTG",
        "CTGACTGAGTCTAGAC",
        "GACTGACTAGTCAGTG",
        "TGCATCGTACGTAGCA",
        "ATGCTGACTGATAGCT",
    ];

    // let encoded_dna: Vec<EncodedDNA> = example.iter().map(|seq| EncodedDNA::new(seq)).collect();
    let example_clone = black_box(example.clone());

    c.bench_function("run_get_balance", |seq| {
        seq.iter_batched(
            || black_box(&example_clone),
            |data| black_box(get_balance(data.to_vec())),
            BatchSize::SmallInput,
        )
    });
    // c.bench_function("run_example", |b| {
    //     b.iter(|| {
    //         black_box(get_balance_as_bits(&encoded_dna));
    //     })
    // });
}

criterion_group! {
name = benches;
config = configure_criterion();
targets=dna_benchmark,
}
criterion_main!(benches);
