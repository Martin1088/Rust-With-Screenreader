use core::str;
use std::u8;

use super::bits_handler::EncodedDNA;

pub fn get_balance(sequences: Vec<&str>) -> Option<Vec<Vec<u8>>> {
    if sequences.is_empty() {
        println!("There are no sequences as parameter");
        return None;
    }

    let length_base = sequences.iter().map(|seq| seq.len()).max().unwrap_or(0);
    let mut result: Vec<Vec<u8>> = vec![vec![0; 2]; length_base];

    for seq in &sequences {
        for cycle in 0..seq.len() {
            let ch_base = seq.chars().nth(cycle)?;
            if ch_base == 'A' || ch_base == 'C' {
                result[cycle][0] += 1;
            }

            if ch_base == 'T' || ch_base == 'A' {
                result[cycle][1] += 1;
            }
        }
    }
    Some(result)
}

// sequence dna amount 0 - 255 and variable length 0 - 200
pub fn get_balance_as_bits(sequence_encoded: &Vec<EncodedDNA>) -> Vec<Vec<u8>> {
    let max_len = sequence_encoded
        .iter()
        .map(|seq| seq.length)
        .max()
        .unwrap_or(0);
    let mut result: Vec<Vec<u8>> = vec![vec![0, 0]; max_len];
    for seq in sequence_encoded {
        for i in 0..seq.length {
            let bits = get_base_bits_at(&seq.chunks, i);
            if is_red(bits) {
                result[i][0] += 1;
            }
            if is_green(bits) {
                result[i][1] += 1;
            }
        }
    }
    result
}

pub fn display_balance(res: &Vec<Vec<u8>>) {
    println!("-- red / green");
    for (i, cycle) in res.iter().enumerate() {
        let formatted = format!("{}. {} / {}", i + 1, cycle[0], cycle[1]);
        println!("{}", formatted);
    }
}

fn get_base_bits_at(chunks: &Vec<u64>, pos: usize) -> u8 {
    let chunk_index = pos / 32;
    let shift = 2 * (31 - (pos % 32));
    ((chunks[chunk_index] >> shift) & 0b11) as u8
}

#[inline]
fn is_red(bits: u8) -> bool {
    matches!(bits, 0b00 | 0b01)
}
#[inline]
fn is_green(bits: u8) -> bool {
    matches!(bits, 0b00 | 0b11)
}
