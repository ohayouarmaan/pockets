use std::collections::HashMap;

// It should be a hashmap which stores a key which should be a string
// and the value should be SupportedValues enum type
// the supported values can be 
// 1. String,
// 2. Floats,
// 3. LinkedList,
// TODO: Add more datastructures which can be saved.
#[derive(Debug)]
pub struct DataStore<'a> {
    hm: HashMap<&'a str, SupportedValues<'a>>
}

impl<'a> Default for DataStore<'a> {
    fn default() -> Self {
        Self {
            hm: HashMap::new()
        } 
    }

}

#[derive(Debug)]
pub enum SupportedValues<'a> {
    PocketString(&'a str),
    PocketNumber(f64),
}
