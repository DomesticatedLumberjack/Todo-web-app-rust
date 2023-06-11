use std::{env::vars, collections::HashMap};
use dotenv::dotenv;

#[derive(Clone)]
pub struct Config{
    values: HashMap<String, String>
}

impl Config{
    pub fn init() -> Config{
        //Get env
        dotenv().ok();
        let mut new_hash_map: HashMap<String, String> = HashMap::new();

        for (n,v) in vars(){
            new_hash_map.insert(n, v);
        }

        Config{
            values: new_hash_map
        }
    }

    pub fn get(&self, key: &str) -> Option<String>{
        self.values.get(key).cloned()
    }
}