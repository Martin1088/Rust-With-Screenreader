#[derive(Debug)]
pub struct EncodedDNA {
    pub chunks: Vec<u64>,
    pub length: usize,
}

impl EncodedDNA {
    pub fn new(sequence: &str) -> EncodedDNA {
        let mut encoded = Vec::new();
        let mut chunk: u64 = 0;
        let mut count = 0;
        for ch in sequence.chars() {
            chunk = (chunk << 2) | encode_base(ch).unwrap() as u64;
            count += 1;
            if count == 32 {
                encoded.push(chunk);
                chunk = 0;
                count = 0;
            }
        }
        if count > 0 {
            encoded.push(chunk << (2 * (32 - count)));
        };
        EncodedDNA {
            chunks: encoded,
            length: sequence.len(),
        }
    }
}

pub fn encode_base(ch_base: char) -> Option<u8> {
    match ch_base {
        'A' => Some(0b00),
        'C' => Some(0b01),
        'G' => Some(0b10),
        'T' => Some(0b11),
        _ => None,
    }
}
