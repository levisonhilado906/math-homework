use std::collections::HashMap;
use std::str;

fn main() {
    let mut hm = HashMap::new();
    hm.insert("key1", "value1");
    hm.insert("key2", "value2");

    for (k, v) in hm {
        println!("{} => {}", k, v);
    }
}
