use hashmap::robin::AmazingHashMap;
fn main() {
    // let entry=Entry::new(1231,"hej");
    // let serialized = serde_json::to_string(&entry).unwrap();
    // println!("serialized = {}", serialized);

    // println!("Hello, world!");

    // for finding keys that collide for testing

    // let start_key=1;
    // let mut hasher=AHHasher::new();
    // start_key.hash(&mut hasher);
    // let start_hash=hasher.finish()% 8;

    // let mut working_key=2;
    // let mut counter=0;
    // loop{
    //     let mut hasher=AHHasher::new();
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

    //}

    let mut hashmap = AmazingHashMap::<usize, usize>::new();

    for i in 0..14{
        hashmap.insert(i, i*10);
    }
    //println!("{:?}",hashmap.table);
    //println!("{:?}",hashmap.entry_index);
    // hashmap.insert(1, 23);
    // hashmap.insert(9, 231231);
    // hashmap.insert(17, 23);
    // hashmap.insert(21, 423);
    // hashmap.insert(32, 23);
    // hashmap.insert(18, 23);
    // hashmap.insert(42, 23);
    // hashmap.insert(3, 123);
    // println!("{:?}", hashmap.table);
    // println!("{:?}", hashmap.entry_index);
    // println!("------------------------------------------");

    // hashmap.delete(1);

    // println!("{:?}", hashmap.table);
    // println!("{:?}", hashmap.entry_index);
}


