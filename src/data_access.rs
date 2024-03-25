use crate::structs;
use crate::constants::{WORDSETS_PATH};

use std::io;
use std::collections::HashMap;
use std::fs;
use structs::{WordsetInfo};

// Change the return type from io::Result<()> to io::Result<Vec<WordsetInfo>>
pub fn get_all_wordsets() -> io::Result<Vec<WordsetInfo>> {
    let file_content: String = fs::read_to_string(WORDSETS_PATH)?;

    // Attempt to parse the JSON content to a HashMap
    let parsed: Result<HashMap<String, WordsetInfo>, serde_json::Error> = serde_json::from_str(&file_content);

    match parsed {
        Ok(data) => {
            // Instead of printing, collect the values into a Vec<WordsetInfo>
            let wordsets: Vec<WordsetInfo> = data.into_iter().map(|(_key, value)| value).collect();
            // Return the vector
            Ok(wordsets)
        },
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
            // Return an error if parsing fails
            Err(io::Error::new(io::ErrorKind::Other, "Failed to parse JSON"))
        },
    }
}
