use analyse::{
    bits_handler::EncodedDNA,
    check_seq::{display_balance, get_balance, get_balance_as_bits},
};

pub mod analyse;

pub fn run_example() {
    let example = vec!["GTCAGTCA", "AGTAGTAC", "CTCTGACA"];
    let res = get_balance(example).unwrap();
    display_balance(&res);
    // let example = vec![
    //     "GTCAGTCAAGCTAGTC",
    //     "AGTAGTACGCTAGCTA",
    //     "CTCTGACAGTCAAGTC",
    //     "TGCATGCACTGATGCA",
    //     "ATGCATGCAGTCATGC",
    //     "GATCTGACATGCAGTC",
    //     "ACGTACGTGATCGTAC",
    //     "GTAGCTAGTGACTGCA",
    //     "CTAGCTAGGACTAGCT",
    //     "AGCTAGCTGACTAGGC",
    //     "CGATCGATGATCTAGT",
    //     "TGCAATGCAGCTAGTC",
    //     "GCTAGTCAAGCTAGTC",
    //     "ATCGATCGACTAGCTA",
    //     "TGCAGCTAGTGCATAG",
    //     "AGCTGACTGACTAGTG",
    //     "CTGACTGAGTCTAGAC",
    //     "GACTGACTAGTCAGTG",
    //     "TGCATCGTACGTAGCA",
    //     "ATGCTGACTGATAGCT",
    // ];
    // let mut encoded_dna: Vec<EncodedDNA> = Vec::new();
    // for i in example {
    //     encoded_dna.push(EncodedDNA::new(i));
    // }
    // let result_balance = get_balance_as_bits(&encoded_dna);
    //display_balance(&result_balance);
}
