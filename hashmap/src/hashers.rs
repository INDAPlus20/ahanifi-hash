use std::hash::Hasher;

pub struct AHHasher {
    hash: u64,
}

impl AHHasher {
    pub fn new() -> AHHasher {
        AHHasher {
            hash: 5381u64, // VERY NICE PRIME
        }
    }
}

impl Hasher for AHHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.hash = (self.hash << 5)
                .wrapping_add(self.hash)
                .wrapping_add(b as u64);
        }
    }
    fn finish(&self) -> u64 {
        self.hash
    }
}
/*
Use of hasher

let mut hashe r= AHHasher::new()
key.hash(&mut hasher)
let hash = hasher.finish()

 */
