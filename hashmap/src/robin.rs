use super::hashers;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Clone, Copy, Debug)]
pub struct EntryIndex {
    index: usize,
    hash: u64,
}
impl EntryIndex {
    fn new(index: usize, hash: u64) -> EntryIndex {
        EntryIndex { index, hash }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Entry<K, V> {
    pub key: K,
    pub value: V,
}
impl<K, V> Entry<K, V> {
    pub fn new(key: K, value: V) -> Entry<K, V> {
        Entry { key, value }
    }
}

/*
Hashmap that uses 2 vectors. The hash is used to calculate the index in the entry_index vector.
The element at that index tells us at what position the value exists in the hash table. The table gets populated in insertion order.
This leads to less memory usage because of the empty buckets that exist in the ordinary implementation.

Using linear probing and open addressing. Robinhood style.
*/
#[derive(Debug)]
pub struct AmazingHashMap<K, V> {
    capacity: usize,
    pub table: Vec<Option<Entry<K, V>>>,
    pub entry_index: Vec<Option<EntryIndex>>,
    current_index: usize,
    mask: u64, // if the hashtable capacity is a power of 2 instead of mod you can use a bitmask
}

impl<K, V> AmazingHashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> AmazingHashMap<K, V> {
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

    pub fn insert(&mut self, key: K, value: V) {
        let hash = self.hash(&key);
        let mut displacement: usize = 0;
        let mut counter: usize = 0;
        let mut index_to_replace = self.current_index;

        let mod_hash = hash & self.mask;

        loop {
            match self.entry_index[((mod_hash + counter as u64) & self.mask) as usize] {
                Some(entry_index) if entry_index.hash == hash => {
                    match &mut self.table[entry_index.index] {
                        Some(ref mut entry) => {
                            if entry.key == key {
                                entry.value = value;
                                return;
                            }
                        }
                        None => {}
                    }
                }
                Some(ref mut entry_index) => {
                    let entry_displacement: usize = ((hash + counter as u64) as isize
                        - entry_index.hash as isize)
                        .abs() as usize;
                    if displacement > entry_displacement {
                        displacement = entry_displacement;
                        let temp = entry_index.index;
                        entry_index.index = index_to_replace;
                        index_to_replace = temp;
                    }
                }

                None => {
                    self.entry_index[((mod_hash + counter as u64) & self.mask) as usize] =
                        Some(EntryIndex::new(index_to_replace, hash));
                    break;
                }
            }
            displacement += 1;
            counter += 1;
        }

        self.current_index += 1;
        self.table.push(Some(Entry::new(key, value)));

        if 3 * self.current_index >= self.capacity * 2 {
            self.resize();
        }
    }

    pub fn delete(&mut self, key: K) -> Result<(), &str> {
        let hash = self.hash(&key);
        let mut counter = 0;
        let mut displacement = 0;
        let entry_to_delete;
        let pos_to_delete;
        let mod_hash = hash & self.mask;
        {
            loop {
                match self.entry_index[((mod_hash + counter) & self.mask) as usize] {
                    Some(entry_index) if entry_index.hash == hash => {
                        match &self.table[entry_index.index] {
                            Some(entry) => {
                                if entry.key == key {
                                    entry_to_delete = entry_index.index;
                                    pos_to_delete = ((hash + counter) & self.mask) as usize;
                                    break;
                                }
                            }
                            None => return Err("dont know"),
                        }
                    }
                    Some(entry_index) => {
                        let entry_displacement = ((hash + counter as u64) as isize
                            - entry_index.hash as isize)
                            .abs() as u64;
                        if displacement > entry_displacement {
                            return Err("No entry with that key");
                        }
                    }
                    None => {
                        return Err("No entry with that key");
                    }
                }

                counter += 1;
                displacement += 1;
            }
        }

        if self.current_index > 1 {
            {
                self.current_index -= 1;
                let index = self.entry_index[pos_to_delete].unwrap().index;
                self.entry_index[pos_to_delete] = None;

                if index == self.current_index {
                    // if the entry we are deleting is the last element of the table.
                    self.table.pop();
                    return Ok(());
                }

                let new_key = &self.table.last().unwrap().as_ref().unwrap().key;
                let position = self._lookup(&new_key).unwrap();

                // println!("{}", pos_to_delete);
                // println!("new_key {:?}", new_key);
                // println!("position {}", position);

                self.entry_index[position].as_mut().unwrap().index = entry_to_delete;
            }

            self.table.swap_remove(entry_to_delete);
        } else {
            self.table.pop();
            self.entry_index[pos_to_delete] = None;
            self.current_index -= 1;
        }

        Ok(())
    }

    pub fn lookup(&self, key: &K) -> Option<&Entry<K, V>> {
        let hash = self.hash(key);
        let mut displacement: u64 = 0;
        let mut counter: u64 = 0;
        let mod_hash = hash & self.mask;

        loop {
            match self.entry_index[((mod_hash + counter) & self.mask) as usize] {
                Some(entry_index) if entry_index.hash == hash => {
                    match &self.table[entry_index.index] {
                        Some(entry) => {
                            if entry.key == *key {
                                return Some(entry);
                            }
                        }
                        None => return None,
                    }
                }
                Some(entry_index) => {
                    let entry_displacement = ((mod_hash + counter as u64) as isize
                        - entry_index.hash as isize)
                        .abs() as u64;
                    if displacement > entry_displacement {
                        return None;
                    }
                }
                None => {
                    return None;
                }
            }

            counter += 1;
            displacement += 1;
        }
    }

    fn _lookup(&self, key: &K) -> Option<usize> {
        let hash = self.hash(key);
        let mut displacement: u64 = 0;
        let mut counter: u64 = 0;
        let mod_hash = hash & self.mask;

        loop {
            let position = ((mod_hash + counter) & self.mask) as usize;
            match self.entry_index[position] {
                Some(entry_index) if entry_index.hash == hash => {
                    match &self.table[entry_index.index] {
                        Some(entry) => {
                            if entry.key == *key {
                                return Some(position);
                            }
                        }
                        None => return None,
                    }
                }

                Some(entry_index) => {
                    let entry_displacement =
                        ((hash + counter as u64) as isize - entry_index.hash as isize).abs() as u64;
                    if displacement > entry_displacement {
                        return None;
                    }
                }

                None => {
                    println!("none");
                    return None;
                }
            }

            counter += 1;
            displacement += 1;
        }
    }

    fn resize(&mut self) {
        let new_capacity: u64 = (self.capacity << 1).try_into().unwrap(); // double the size of the table
        let mut new_entry_index: Vec<Option<EntryIndex>> = vec![None; new_capacity as usize];

        for option_index in &mut self.entry_index {
            match option_index {
                Some(entry_index) => {
                    let new_hash = entry_index.hash & (new_capacity - 1);
                    std::mem::replace(&mut new_entry_index[new_hash as usize], Some(*entry_index))
                        .unwrap();
                }
                None => {}
            }
        }

        self.entry_index = new_entry_index;
        self.capacity = new_capacity as usize;
        self.mask = (self.capacity - 1) as u64;
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = hashers::AHHasher::new();
        key.hash(&mut hasher);
        hasher.finish() //& self.mask // so we get the hash within the capacity
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{Hash, Hasher};

    use super::hashers;
    use super::AmazingHashMap;

    #[test]
    fn basic_no_collision_insert() {
        let mut hashmap = AmazingHashMap::<usize, usize>::new();
        let key = 1usize;
        let value = 1337usize;

        let mut hasher = hashers::AHHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() & hashmap.mask;

        hashmap.insert(key, value);

        let index = hashmap.entry_index[hash as usize].unwrap().index;
        let returned_val = hashmap.table[index].unwrap().value;

        assert_eq!(value, returned_val);

        let key = 42usize;
        let value = 1231;

        let mut hasher = hashers::AHHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() & hashmap.mask;

        hashmap.insert(key, value);

        let index = hashmap.entry_index[hash as usize].unwrap().index;
        let returned_val = hashmap.table[index].unwrap().value;

        assert_eq!(value, returned_val);
    }

    #[test]
    fn test_update_value_no_collision() {
        let mut hashmap = AmazingHashMap::<usize, usize>::new();
        let key = 1usize;

        hashmap.insert(key, 1337);

        hashmap.insert(key, 2021);

        let mut hasher = hashers::AHHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() & hashmap.mask;

        let returned_val = hashmap.table[hashmap.entry_index[hash as usize].unwrap().index]
            .unwrap()
            .value;
        assert_eq!(2021, returned_val);
    }

    #[test]
    fn test_lookup() {
        let mut hashmap = AmazingHashMap::<usize, usize>::new();
        hashmap.insert(321, 4124);

        assert_eq!(4124, hashmap.lookup(&321).unwrap().value);
        assert!(hashmap.lookup(&31231).is_none())
    }
    #[test]
    fn test_resize() {
        let mut hashmap = AmazingHashMap::<usize, usize>::new();
        for i in 0..14 {
            hashmap.insert(i, i * 10);
        }
        for i in 0..14 {
            println!("{}", i);
            assert!(hashmap.lookup(&i).unwrap().value == i * 10)
        }
    }

    #[test]
    fn test_delete() {
        let mut hashmap = AmazingHashMap::<usize, usize>::new();
        for i in 0..10 {
            hashmap.insert(i, i * 10);
        }
        for i in 0..10 {
            hashmap.delete(i).unwrap();
        }
    }
}
