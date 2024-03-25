pub mod structs;
mod data_access;
mod constants;

extern crate serde;
extern crate serde_json;

use std::{
  io::{self, BufRead, BufReader}, 
  process::exit,
  path::Path, 
  fs::File 
};

use serde::{Deserialize, Serialize};
use structs::{WordsetInfo};
use constants::{TITLE};
use data_access::{get_all_wordsets};
use colored::*;


fn main() { 
  let _ = main_menu();













  // let _ = get_wordsets();

  // Word Set Selection
  // loop {
  //   io::stdout().flush().unwrap(); // Ensure the print statement is displayed before getting input

  //   let mut input = String::new(); // Buffer to store input
  //   io::stdin()
  //     .read_line(&mut input)
  //     .expect("Failed to read line"); // Read input

  //   match input.trim().parse::<u32>() { // Attempt to parse input as u32
  //     Ok(number) => {
  //       println!("You entered: {}", number);
  //       break; // Exit the loop if input is a valid number
  //     },
  //     Err(_) => {
  //       println!("Error: Please enter a valid number.");
  //       // The loop will continue, asking the user for input again
  //     },
  //   }
  // }
}

fn main_menu() -> io::Result<()> {  
  clear_terminal();
  display_title()?;
  
  println!("Welcome to Hangman {}", "v0.1".blue());
  println!("Please select your option\n");
  println!("• Start\n• Wordsets\n• Help\n• Quit");
  
  loop {
    let mut choice: String = String::new();

    io::stdin().read_line(&mut choice)
      .expect("Failed to read line");

    choice = choice.trim().to_lowercase();

    match choice.as_str() {
      "start"     | "s" => start_game(),
      "wordsets"  | "w" => wordsets_menu(),
      "help"      | "h" => help_menu(),
      "quit"      | "q" => {
        println!("\n{}", "Exiting...".yellow());
        exit(0);
      },
      _ => println!("{}", "Invalid option, type Help or h for help".red()),
    }
  }

  fn display_title() -> io::Result<()> {
    let file: File = File::open(Path::new(TITLE))?;
    let reader: BufReader<File> = BufReader::new(file);

    // Prints Hangman ASCII art
    for line in reader.lines() {
      println!("{}", line?.magenta());
    }

    Ok(())
  }
}

fn start_game() {

}

fn wordsets_menu() {
  clear_terminal();

  println!("• View Wordsets\n• Load Wordset\n• Delete Wordsets");
  println!("\nPress the {} key to go back to the main menu", "Enter".yellow());

  loop {
    let mut choice: String = String::new();

    io::stdin().read_line(&mut choice)
      .expect("Failed to read line");

    choice = choice.trim().to_lowercase();
    
    if choice.trim().is_empty() {
      break;
    }
    else {
      match choice.as_str() {
        "view"   | "v" => view_wordsets(),
        "load"   | "l" => {},
        "delete" | "d" => {},
        _ => {}    
      }
    }
  }

  let _ = main_menu();

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
}

fn help_menu() {
  clear_terminal();
  
  // ================================================================================
  println!("List of commands (Commands are not case sensitive):\n");

  println!("{}", "Main Menu Commands:".cyan());
  println!(" {} : Starts the game", "Start | S".yellow());
  println!(" {} : List of helpful commands", "Help | H".yellow());
  println!(" {} : Upload or remove wordsets", "Wordsets | W".yellow());
  println!(" {} : Quits the game", "Quit | Q".yellow());

  println!("\n{}", "Wordsets Menu Commands:".cyan());
  println!(" {} : View all loaded wordsets", "View | V".yellow());
  println!(" {} : Load a new wordset into game", "Load | L".yellow());
  println!(" {} : Delete a wordset from game", "Delete | D".yellow());

  println!("\n{}", "Universal Commands:".cyan());
  println!(" {} : Quits the game at any point", "Ctrl + C".yellow());

  println!("\nPress the {} key to go back to the main menu", "Enter".yellow());
  // ================================================================================

  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().is_empty() {
      break;
    }
  }

  let _ = main_menu();
}

fn clear_terminal() {
  print!("{}[2J", 27 as char);
}