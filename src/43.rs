use std::collections::HashMap;

fn main() {
    let mut data: HashMap<String, i32> = HashMap::new();

    // Example of how to add a value to the map
    data.insert(String::from("key1"), 5);
    data.insert(String::from("key2"), 10);
    data.insert(String::from("key3"), 15);

    for (k, v) in &data {
        println!("Key: {} | Value: {}", k, v);
    }
}
