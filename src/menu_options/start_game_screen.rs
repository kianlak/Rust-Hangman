use crate::{data_access::{access_wordset_by_key, collect_all_wordsets}, game::game::load_game, menu_options::menu_shared::{clear_terminal, go_to_main_menu}, structs::WordsetInfo};

use std::{collections::HashMap, io::{self, Write}};
use colored::*;


pub fn start_game() {
  clear_terminal();

  select_wordset_screen();
}

fn select_wordset_screen() {
  let wordsets: Result<HashMap<String, WordsetInfo>, io::Error> = collect_all_wordsets();
  let mut max_index: usize = 0; 

  println!("{}", "Select the number for the wordset you want to use:".blue());

  match wordsets {
    Ok(wordsets) => {
      for (key, wordset) in wordsets.iter() {
        match key.parse::<usize>() {
          Ok(parsed_number) => {
            if parsed_number > max_index {
              max_index = parsed_number;
            }
          },
          Err(e) => {
            println!("Failed to parse the string into a number: {:?}", e);
          }
      }

        println!("{} {}", 
          key.magenta(), 
          wordset.name.green(), 
        );
      }
    },
    Err(e) => {
      println!("{} {}", "Failed to get wordsets: ".red(), e);
    }
  }

  loop {
    let mut input: String = String::new();
    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");
  
    let input: &str = input.trim();
    
    if input.is_empty() {
      go_to_main_menu();
    }
    
    if is_in_range(input, max_index) {
      let path = access_wordset_by_key(input);
      
      match path {
        Ok(path) => {
          println!("{}", path);
          load_game(&path);
        },
        Err(error_message) => println!("Error: {}", error_message),
      }
    }
  }
}

fn is_in_range(input: &str, size: usize) -> bool {
  if !is_integer(input) {
    return false;
  }

  match input.parse::<usize>() {
    Ok(number) => {
      if number <= size {        
        return true;
      } 
      else {
        println!("{} {} {}.", 
          number.to_string().yellow(),
          "is out of range, please enter a number between 1 and".red(),
          size.to_string().yellow()
        );

        return false;
      }
    },
    Err(_) => {
      println!("{}", "Something unexpected happened".red());

      return false;
    }
  }
}

fn is_integer(input: &str) -> bool {
  match input.parse::<i32>() {
    Ok(num) => {
      if num <= 0 {
        println!("{}", "That is not a proper index, please try again.".red());
        println!("You can also press {} to go back to the main menu\n", "Enter".yellow());

        return false;
      }

      return true;
    }
    Err(_) => {
      println!("{}", "That is not an integer, please try again".red());
      println!("You can also press {} to go back to the main menu\n", "Enter".yellow());

      return false;
    }
  }
}