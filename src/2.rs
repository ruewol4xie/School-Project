use std::collections::HashMap;

fn main() {
    let mut my_map = HashMap::new();
    my_map.insert("key1", "value1");
    my_map.insert("key2", "value2");
    my_map.insert("key3", "value3");
    println!("{:?}", my_map);
}
