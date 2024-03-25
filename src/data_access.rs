use crate::structs;
use crate::constants::{WORDSETS_PATH};

use std::io;
use std::collections::HashMap;
use std::fs;
use structs::{WordsetInfo};


pub fn get_wordsets() -> io::Result<()> {
  let file_content: String = fs::read_to_string(WORDSETS_PATH)?;

  let parsed: Result<HashMap<String, WordsetInfo>, serde_json::Error> = serde_json::from_str(&file_content);

    match parsed {
        Ok(data) => {
            // Iterate over the HashMap, converting the keys to integers
            for (key, value) in data {
                let key_as_number: Result<i32, _> = key.parse();
                match key_as_number {
                    Ok(num) => println!("{}. name: {}, path: {}", num, value.name, value.path),
                    Err(_) => println!("Error: Key '{}' is not a valid number.", key),
                }
            }
        },
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to parse JSON"));
        },
    }

    Ok(())
}