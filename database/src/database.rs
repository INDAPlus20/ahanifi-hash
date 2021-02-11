use std::{
    fs::OpenOptions,
    hash::Hash,
    io::BufReader,
};

use hashmap::hashmap::{AmazingHashMap, Entry};
use serde::{Deserialize, Serialize};


const DEFAULT_BACKUP_PATH: &str = "save/backup.json";

pub struct JaySonDB<K, V> {
    pub hashmap: AmazingHashMap<K, V>,
    backup_file: String,
}

impl<'de, K, V> JaySonDB<K, V>
where
    K: Serialize + Deserialize<'de> + Hash + Eq,
    V: Serialize + Deserialize<'de>,
{
    pub fn new(backup_file: Option<String>) -> JaySonDB<K, V> {
        let backup_file = match backup_file {
            Some(path) => path,
            None => DEFAULT_BACKUP_PATH.to_string(),
        };

        let hashmap = AmazingHashMap::<K, V>::new();

        let mut db = JaySonDB {
            hashmap,
            backup_file,
        };
        db.load_file();
        db
    }

    fn load_file(&mut self) -> Result<(), String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(&mut self.backup_file)
            .expect("couldn't open file");

        let reader = BufReader::new(file);

        let deserializer = serde_json::Deserializer::from_reader(reader);
        let iterator = deserializer.into_iter::<Entry<K, V>>();
        for item in iterator {
            match item {
                Ok(entry) => self.hashmap.insert(entry.key, entry.value),
                Err(e) => return Err(format!("Couldn't load database from file. Error {:?}", e)),
            }
        }
        //hashmap.insert(entry.key, entry.value);
        Ok(())
    }

    fn full_backup(&mut self) {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .append(false)
            .open(&mut self.backup_file)
            .expect("couldn't open file");

        for el in &self.hashmap.table {
            match el {
                Some(entry) => serde_json::to_writer(&file, &entry).unwrap(),
                None => continue,
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.hashmap.insert(key, value);
        self.full_backup();
    }

    pub fn delete(&mut self, key: K) {
        self.hashmap.delete(key).unwrap();
        self.full_backup();
    }

    pub fn lookup(&self, key: K) -> Option<&Entry<K, V>> {
        self.hashmap.lookup(&key)
    }
    pub fn list_all(&self) -> &Vec<Option<Entry<K, V>>> {
        &self.hashmap.table
    }
}
