use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn siphash_example<T: Hash>(data: &T) -> u64 {
    let mut hasher = DefaultHasher::new(); // Uses SipHash-2-4 by default
    data.hash(&mut hasher);
    hasher.finish() // Returns the hash value
}
