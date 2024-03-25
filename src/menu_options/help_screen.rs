use std::io::{self, Write};
use colored::*;

use crate::menu_options::menu_shared::go_to_main_menu;

use super::menu_shared::clear_terminal;

pub fn help_menu() {
  clear_terminal();
  
  // ================================================================================
  println!("{}", "Hangman Rules:".cyan());
  println!("The word to guess is represented by a row of underscores representing each character of the word.\nTry guessing the word before the hangman drawing is complete");
  
  println!("\n{}","List of commands (Commands are not case sensitive):".green());

  println!("{}", "Main Menu Commands:".cyan());
  println!(" {} : Starts the game", "Start | S".yellow());
  println!(" {} : List of helpful commands", "Help | H".yellow());
  println!(" {} : Upload or remove wordsets", "Wordsets | W".yellow());
  println!(" {} : Quits the game", "Quit | Q".yellow());

  println!("\n{}", "Wordsets Menu Commands:".cyan());
  println!(" {} : View all loaded wordsets", "View Wordsets | View | V".yellow());
  println!(" {} : Load a new wordset into game", "Load Wordsets | Load | L".yellow());
  println!(" {} : Delete a wordset from game", "Delete Wordsets | Delete | D".yellow());

  println!("\n{}", "Universal Commands:".cyan());
  println!(" {} : Quits the game at any point", "Ctrl + C".yellow());

  println!("\nPress the {} key to go back to the main menu", "Enter".yellow());
  // ================================================================================

  loop {
    let mut input: String = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");

    if input.trim().is_empty() {
      break;
    }
  }

  go_to_main_menu();
}