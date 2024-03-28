use std::io::Write;
use std::{fs, io};

use colored::*;

use crate::menu_options::menu_shared::{clear_terminal, go_to_main_menu};
use crate::data_access::select_random_word;

fn game(word_to_guess: String) {
  
  let hangman_art: Vec<String> = load_hangman_art();
  
  for art in hangman_art.iter() {
    println!("{}", art);
  }
  
  let mut art_index: usize = 0;
  
  let secret_word: String = word_to_guess;
  
  let mut guessed_word = secret_word
  .chars()
  .map(|c| if c == ' ' { ' ' } else { '_' })
  .collect::<String>();

  let mut guessed_letters: Vec<char> = Vec::new();

  while guessed_word.contains('_')  {
    clear_terminal();
    if art_index == 6 {
      lose_screen(&secret_word);
    }

    println!("{}", hangman_art[art_index]);
    println!("Guessed word so far: {}", guessed_word);
    println!("Guessed letters: {:?}", guessed_letters);
    print!("Guess a letter: ");

    io::stdout().flush().unwrap();

    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: char = guess.trim().chars().next().unwrap_or('_').to_lowercase().next().unwrap();

    if guessed_letters.contains(&guess) || guess == ' ' {
      println!("Invalid guess or already guessed. Try again.");
      continue;
    }

    guessed_letters.push(guess);

    let mut found: bool = false;

    for (i, c) in secret_word.chars().enumerate() {
      if c.to_lowercase().next().unwrap() == guess {
        guessed_word.replace_range(i..=i, &guess.to_string());
        found = true;
      }
    }

    if found {
      println!("Correct!");
    } else {
      art_index += 1;
      println!("Sorry, that letter is not in the word.");
    }
  }

  win_screen(secret_word);
}

pub fn load_game(wordset_path: &str) {  
  match select_random_word(wordset_path) {
    Ok(word) => {
      game(word);
    },
    Err(_e) => go_to_main_menu(),
  }
}

fn load_hangman_art() -> Vec<String> {
  let mut hangman_art: Vec<String> = Vec::new();

  for i in 0..=6 {
    let file_name = format!("resources/ascii_art/hangman_{}.txt", i);
    
    match fs::read_to_string(&file_name) {
      Ok(contents) => {
        hangman_art.push(contents);
      },
      Err(e) => {
        println!("Error reading {}: {}", file_name, e);
      },
    }
  }

  return hangman_art;
}

fn lose_screen(secret_word: &String) {
  clear_terminal();

  println!("You Lose, the word was: {}", secret_word.red());

  loop {
    println!("\nPress the {} key to go back to the main menu", "Enter".yellow());
    let mut input: String = String::new();
    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");
  
    let input: &str = input.trim();
    
    if input.is_empty() {
      go_to_main_menu();
    }
  }
}

fn win_screen(secret_word: String) {
  clear_terminal();

  println!("Congratulations! You guessed the word: {}", secret_word.green());

  loop {
    println!("\nPress the {} key to go back to the main menu", "Enter".yellow());
    let mut input: String = String::new();
    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");
  
    let input: &str = input.trim();
    
    if input.is_empty() {
      go_to_main_menu();
    }
  }
}