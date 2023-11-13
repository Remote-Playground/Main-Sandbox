use std::collections::HashMap;
use std::io::Result;

fn main() -> Result<()> {
    let mut data: HashMap<String, String> = HashMap::new();
    insert_data_to_hashmap(&mut data);
    display_data(&mut data);
    validate_key_value(&data, "Data structure".to_string());
    display_hashmap_length(&data);
    Ok(())
}

fn insert_data_to_hashmap(data: &mut HashMap<String, String>) {
    data.insert("Data structure".to_string(), "Hashmap".to_string());
}

fn display_data(data: &mut HashMap<String, String>) {
    for (key, value) in data.iter_mut() {
        println!("> {}: {}", key, value);
    }
}

fn validate_key_value(data: &HashMap<String, String>, key: String) {
    if data.contains_key(&key) {
        println!("Contains key: {}", key);
        let key_value = data.get(&key).unwrap();
        println!("Key value: {:?}", key_value);
    }
}

fn display_hashmap_length(data: &HashMap<String, String>) {
    println!("Length: {}", data.len());
}