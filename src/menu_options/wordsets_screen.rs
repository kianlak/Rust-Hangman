use crate::{data_access::{add_wordset, check_if_txt_exists, get_all_wordsets}, structs::WordsetInfo};
use colored::*;
use std::io::{self, Write};

use super::menu_shared::{clear_terminal, go_to_main_menu};

pub fn wordsets_menu() {
  clear_terminal();

  println!("• View Wordsets\n• Load Wordset\n• Delete Wordsets");
  println!("\nPress the {} key to go back to the main menu", "Enter".yellow());

  loop {
    let mut choice: String = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice)
      .expect("Failed to read line");

    choice = choice.trim().to_lowercase();
    
    if choice.is_empty() {
      go_to_main_menu();
    }
    else {
      match choice.as_str() {
        "viewwordsets"    | "view"    | "v" => view_wordsets(),
        "loadwordsets"    | "load"    | "l" => load_wordsets(),
        "deletewordsets"  | "delete"  | "d" => {},
        _ => {
          println!("{} {} {}", 
            "\nInvalid input, go to help for more info.\nPress".red(),
            "Enter".yellow(),
            "to exit".red()
          )
        }
      }
    }
  }
}

fn view_wordsets() {
  clear_terminal();

  let wordsets: Result<Vec<WordsetInfo>, io::Error> = get_all_wordsets();
  
  println!("Your Loaded Wordsets:");

  match wordsets {
    Ok(wordsets) => {
      for(index, wordset) in wordsets.iter().enumerate() {
        let index_str = format!("{}.", index + 1).to_string();
        
        println!("{} {}, \n   |_ Path: {}", 
          index_str.magenta(), 
          wordset.name.green(), 
          wordset.path.yellow()
        );
      }
    },
    Err(e) => {
      println!("{} {}", "Failed to get wordsets: ".red(), e);
    }
  }

  println!("\nPress the {} key to go back to the main menu", "Enter".yellow());
}

fn load_wordsets() {
  clear_terminal();

  println!("{}", "Instructions:".blue());
  println!("To load in a new wordset make sure to put a {} file of words separated by {} in the {} directory (txt file should not have spaces in its name).\nFrom here just enter the name of the {} file you want to upload and the {} of the wordset, then it will be registered", 
    ".txt".yellow(), 
    "newlines".yellow(),
    "resources/wordsets/".yellow(),
    ".txt".yellow() ,
    "name".yellow()
  );

  let txt_path: String = register_txt_file();

  if txt_path.is_empty() {
    go_to_main_menu();
  }

  let wordset_name: String = register_wordset_name();

  if wordset_name.trim().is_empty() {
    go_to_main_menu();
  }
  else {
    let trimmed_wordset_name = wordset_name.trim();
    let _ = add_wordset(trimmed_wordset_name, &txt_path);

    println!("\nPress the {} key to go back to the main menu", "Enter".yellow());

    loop {
      let mut input: String = String::new();

      io::stdout().flush().unwrap();
      io::stdin().read_line(&mut input)
        .expect("Failed to read line");

      if input.trim().is_empty() {
        go_to_main_menu();
        break;
      }
    }
  }
}

fn register_txt_file() -> String {
  println!("\n{}", "Enter the name of the .txt file you want to register".cyan());

  loop{
    let mut txt_name: String = String::new();
    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut txt_name)
      .expect("Failed to read line");

    let txt_name: &str = txt_name.trim();
    
    if txt_name.is_empty() {
      return "".to_string();
    }

    let (txt_path,  txt_found) = check_if_txt_exists(txt_name);

    if txt_found {
      println!("{} {} {}", 
        "File".green(),
        format!("{}.txt", &txt_name).purple(),
        "found".green()
      );

      return txt_path;
    }
    else {
      println!("{} {} {}", 
        "File path".red(), 
        txt_path.purple(),
        "does not exist, please try again".red()
      );
      println!("You can also press {} to go back to the main menu\n", "Enter".yellow());
    }
  }
}

fn register_wordset_name() -> String {
  let mut wordset_name: String = String::new();
  
  println!("\n{}", "What do you want to name this wordset (Spaces are permitted):".blue());

  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut wordset_name)
    .expect("Failed to read line");

  return wordset_name;
}