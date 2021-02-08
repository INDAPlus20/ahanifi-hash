use core::panic;
use std::{hash::{self, Hash, Hasher}, ops::Index, usize};
use serde::{Deserialize,Serialize};
use serde_json;
fn main() {
    // let entry=Entry::new(1231,"hej");
    // let serialized = serde_json::to_string(&entry).unwrap();
    // println!("serialized = {}", serialized);

    // println!("Hello, world!");


    // for finding keys that collide for testing

    // let start_key=1;
    // let mut hasher=DJHasher::new();
    // start_key.hash(&mut hasher);
    // let start_hash=hasher.finish()% 8;

    // let mut working_key=2;
    // let mut counter=0;
    // loop{
    //     let mut hasher=DJHasher::new();
    //     working_key.hash(&mut hasher);
    //     let working_hash=hasher.finish() % 8;
    //     if working_hash== start_hash{
    //         println!("{}",working_key);
    //         counter+=1;
      
    //     }
    //     if counter > 10{
    //         break;
    //     }
    //     working_key+=1;
        

    // }

    let mut hashmap=AmazingHashMap::<usize,usize>::new();
    // all these keys have colliding hashes
    hashmap.insert(1, 23);
    hashmap.insert(9, 231231);
    hashmap.insert(17, 23);
    hashmap.insert(1, 423);

    println!("{:?}",hashmap.table);
    println!("{:?}",hashmap.entry_index)
    
}

struct SDBMHasher{
    hash:u64
}

impl SDBMHasher{
    fn new() -> SDBMHasher{
        SDBMHasher{
            hash:0
        }
    }
}

impl Hasher for SDBMHasher{
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

#[derive(Clone, Copy, Debug)]
struct EntryIndex {
    index: usize,
    hash: u64,
}
impl EntryIndex {
    fn new(index: usize, hash: u64) -> EntryIndex {
        EntryIndex { index, hash }
    }
}

#[derive(Clone, Copy, Debug,Serialize,Deserialize)]
struct Entry<K, V> {
    key: K,
    value: V,
}
impl<K, V> Entry<K, V> {
    fn new(key: K, value: V) -> Entry<K, V> {
        Entry { key, value }
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


    fn insert(&mut self, key:K, value: V) {
        let hash = self.hash(&key);
        let mut displacement:usize=0;
        let mut counter:usize=0;
        let mut index_to_replace=self.current_index;

        loop{
            match self.entry_index[((hash +counter as u64) & self.mask) as usize] {
                Some(entry_index) if entry_index.hash == hash => {
                    println!("Same Hash");
                    match &mut self.table[entry_index.index] {
                        Some(ref mut entry)=>{ 
                            if entry.key==key{
                                entry.value=value;
                                return;
                            }
                        },
                        None=>{},
                    }
                        
                },
                Some(ref mut entry_index)=>{
                    println!("Not same hash");
                    let entry_displacement=(hash as usize+counter) - entry_index.hash as usize;
                    if displacement > entry_displacement {
                        displacement=entry_displacement;
                        let temp=entry_index.index;
                        entry_index.index=index_to_replace;
                        index_to_replace=temp;
                    }

                },

                None => {
                    self.entry_index[((hash+counter as u64)&self.mask) as usize] = Some(EntryIndex::new(index_to_replace, hash));
                    break;
                },
            }
            displacement+=1;
            counter+=1;
        }
        
        self.current_index += 1;
        self.table.push(Some(Entry::new(key, value)));
        
    }

    fn delete(&mut self, key: K) {
        let hash = self.hash(&key);
    }

    fn lookup(&self, key: &K) ->Option<&Entry<K,V>>{
        let hash=self.hash(key);
        let mut displacement:u64=0;
        let mut counter:u64=0;


        loop{
            match self.entry_index[((hash +counter)& self.mask) as usize]{
                Some(entry_index) if entry_index.hash==hash => {
                    match  &self.table[entry_index.index]{
                        Some(entry)=>{
                            if entry.key==*key{
                                return Some(entry);
                            }
                        }
                        None=> return None,
                        _=>{},
                    }
                },
                None=>{
                    return None;
                },
                _=>{},
            }

            counter+=1;
            displacement+=1;
        }

        None
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = DJHasher::new();
        key.hash(&mut hasher);
        hasher.finish() & self.mask // so we get the hash within the capacity
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{Hash, Hasher};

    use crate::{AmazingHashMap, DJHasher};

    #[test]
    fn basic_no_collision_insert() {
        let mut hashmap = AmazingHashMap::<usize, usize>::new();
        let key = 1usize;
        let value = 1337usize;

        let mut hasher = DJHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() & hashmap.mask;

        hashmap.insert(key, value);

        let index = hashmap.entry_index[hash as usize].unwrap().index;
        let returned_val = hashmap.table[index].unwrap().value;

        assert_eq!(value, returned_val);

        let key = 42usize;
        let value = 1231;

        let mut hasher = DJHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() & hashmap.mask;

        hashmap.insert(key, value);

        let index = hashmap.entry_index[hash as usize].unwrap().index;
        let returned_val = hashmap.table[index].unwrap().value;

        assert_eq!(value, returned_val);
    }

    #[test]
    fn test_update_value_no_collision(){
        let mut hashmap = AmazingHashMap::<usize, usize>::new();
        let key = 1usize;

        hashmap.insert(key, 1337);

        hashmap.insert(key, 2021);

        let mut hasher = DJHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() & hashmap.mask;

        let returned_val=hashmap.table[hashmap.entry_index[hash as usize].unwrap().index].unwrap().value;
        assert_eq!(2021,returned_val);

    }


    #[test]
    fn test_lookup(){
        let mut hashmap=AmazingHashMap::<usize, usize>::new();
        hashmap.insert(321, 4124);

        assert_eq!(4124,hashmap.lookup(&321).unwrap().value);
        assert!(hashmap.lookup(&31231).is_none())
        
    }


}
