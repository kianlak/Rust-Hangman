use crate::constants::TITLE;
use colored::*;
use std::{fs::File, io::{self, BufRead, BufReader, Write}, path::Path, process::exit};

use super::{menu_shared::clear_terminal, help_screen::help_menu, start_game_screen::start_game, wordsets_screen::wordsets_menu};

pub fn main_menu() -> io::Result<()> {  
  clear_terminal();
  display_title()?;
  
  println!("Welcome to Hangman {}", "v1.0".blue());
  println!("Please select your option\n");
  println!("• Start\n• Wordsets\n• Help\n• Quit");
  
  loop {
    let mut choice: String = String::new();

    io::stdout().flush().unwrap();
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
}

fn display_title() -> io::Result<()> {
  let file: File = File::open(Path::new(TITLE))?;
  let reader: BufReader<File> = BufReader::new(file);

  // Prints Hangman ASCII art
  for line in reader.lines() {
    println!("{}", line?.bright_magenta());
  }

  Ok(())
}