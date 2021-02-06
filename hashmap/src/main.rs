use std::hash::{Hash, Hasher};

fn main() {
    println!("Hello, world!");
}

struct DJHasher {
    hash: u64,
}

impl DJHasher {
    fn new() -> DJHasher {
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

#[derive(Clone, Copy,Debug)]
struct EntryIndex {
    index:usize,
    hash:u64,
}
impl EntryIndex {
    fn new(index:usize,hash:u64) -> EntryIndex{
        EntryIndex{
            index,
            hash
        }
    }
}

#[derive(Clone, Copy,Debug)]

struct Entry<K,V>{
    key:K,
    value:V,
}
impl <K,V> Entry<K,V> {
    fn new(key:K,value:V) -> Entry<K,V>{
        Entry{
            key,
            value
        }
    }
}

/*
Hashmap that uses 2 vectors. The hash is used to calculate the index in the entry_index vector.
The element at that index tells us at what position the value exists in the hash table. The table gets populated in insertion order.
This leads to less memory usage because of the empty buckets that exist in the ordinary implementation.

Using linear probing and open addressing. Robinhood style.
*/
struct AmazingHashMap<K, V> {
    capacity: usize,
    table: Vec<Option<Entry<K, V>>>,
    entry_index: Vec<Option<EntryIndex>>,
    current_index: usize,
    mask: u64, // if the hashtable capacity is a power of 2 instead of mod you can use a bitmask
}

impl<K, V> AmazingHashMap<K, V>
where
    K: Hash + Eq,
{
    fn new() -> AmazingHashMap<K, V> {
        let capacity = 8usize;
        let mask = capacity as u64 - 1;

        let entry_index = vec![None; 8];
        let table = Vec::with_capacity(capacity);
        let current_index: usize = 0;

        AmazingHashMap {
            entry_index,
            table,
            capacity,
            mask,
            current_index,
        }
    }

    
    fn insert(&mut self,key:K,value: V){
        let mut hasher = DJHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() & self.mask; // so we get the hash within the capacity. 

        match self.entry_index[hash as usize] {
            Some(entry_index)=>{
                match &self.table[entry_index.index] {
                    Some(entry)=>{
                        
                    },
                    None=>{
                        
                    }
                }
            },
            None=>{
                self.entry_index[hash as usize]=Some(EntryIndex::new(self.current_index,hash));
                self.current_index+=1;
                self.table.push(Some(Entry::new(key, value)))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::hash::{Hash, Hasher};

    use crate::{AmazingHashMap, DJHasher};

    #[test]
    fn basic_no_collision_insert() {
        let mut hashmap=AmazingHashMap::<usize,usize>::new();
        let key =1usize;
        let value=1337usize;

        let mut hasher=DJHasher::new();
        key.hash(&mut hasher);
        let hash=hasher.finish()&hashmap.mask;

        hashmap.insert(key, value);

        let index = hashmap.entry_index[hash as usize].unwrap().index;
        let returned_val=hashmap.table[index].unwrap().value;

        assert_eq!(value,returned_val);
    }
}
