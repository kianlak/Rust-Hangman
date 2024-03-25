use crate::structs;
use crate::constants::WORDSETS_PATH;

use std::io;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use structs::WordsetInfo;

// Returns all the wordsets that are loaded into the game
pub fn get_all_wordsets() -> io::Result<Vec<WordsetInfo>> {
  let file_content: String = fs::read_to_string(WORDSETS_PATH)?;

  // Attempt to parse the JSON content to a HashMap
  let parsed: Result<HashMap<String, WordsetInfo>, serde_json::Error> = serde_json::from_str(&file_content);

  match parsed {
    Ok(data) => {
      let wordsets: Vec<WordsetInfo> = data.into_iter().map(|(_key, value)| value).collect();

      Ok(wordsets)
    },
    Err(e) => {
      println!("Failed to parse JSON: {}", e);
      Err(io::Error::new(io::ErrorKind::Other, "Failed to parse JSON"))
    },
  }
}

pub fn check_if_txt_exists(file_name: &str) -> (String, bool) {
  let path: String = format!("resources/wordsets/{}.txt", file_name);
  
  if Path::new(&path).exists() {
    return (path, true);
  } else {
    return (path, false);
  }
}