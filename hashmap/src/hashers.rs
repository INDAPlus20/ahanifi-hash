use std::hash::Hasher;

struct SDBMHasher {
    hash: u64,
}

impl SDBMHasher {
    fn new() -> SDBMHasher {
        SDBMHasher { hash: 0 }
    }
}

impl Hasher for SDBMHasher {
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

pub struct DJHasher {
    hash: u64,
}

impl DJHasher {
    pub fn new() -> DJHasher {
        DJHasher {
            hash: 5381u64, // 5381 is supposedly a very good number
        }
    }
}

impl Hasher for DJHasher {
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

let mut hashe r= DJHasher::new()
key.hash(&mut hasher)
let hash = hasher.finish()

 */
