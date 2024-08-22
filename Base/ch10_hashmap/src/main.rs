use std::collections::HashMap;

fn main() {
    println!("Hello, hashmap!");
    // initialize an empty hashmap
    let mut map = HashMap::new();
    // add key-value pairs to the hashmap
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    // map.insert("4", "four");

    println!("map: {:#?}", map);
    println!("----------1------------");
    // display the contents of the hashmap
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // access a value by its key
    println!("value of key 2: {}", map.get(&2).unwrap());
    println!("value of key 2: {:?}", map.get(&2));
    // println!("\n4 = {:#?}", map.get("4"));

    println!("----------2------------");
    // update a value by its key
    map.insert(2, "two updated");
    println!("map: {:#?}", map);

    println!("----------3------------");
    // remove a key-value pair by its key
    map.remove(&2);
    println!("map: {:#?}", map);

    println!("----------4------------");
    // insert a key-value pair if the key does not already exist
    map.entry(4).or_insert("four");
    println!("map: {:#?}", map);
}
