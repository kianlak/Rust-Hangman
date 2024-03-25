use std::io::Write;
use crate::constants::WORDSETS_PATH;
use crate::structs;

use std::{fs::File, io};
use std::collections::HashMap;
use std::fs;
use colored::Colorize;
use serde_json::{Value, Map};
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


pub fn add_wordset(name: &str, path: &str) -> io::Result<()> {
  let file_contents: String = fs::read_to_string(WORDSETS_PATH)?;
  let mut data: Map<String, Value> = serde_json::from_str(&file_contents)?;

  let new_index: usize = data.len() + 1;
  let new_index_str: String = new_index.to_string();

  let new_wordset: WordsetInfo = WordsetInfo {
    name: name.trim().to_string(),
    path: path.to_string(),
  };

  data.insert(new_index_str, serde_json::to_value(new_wordset)?);

  let mut file: File = File::create(WORDSETS_PATH)?;
  write!(file, "{}", serde_json::to_string_pretty(&data)?)?;

  let success_file_name: String = name.to_string();

  println!("{} {}",
    success_file_name.green(),
    "has been loaded".green()
  );

  Ok(())
}