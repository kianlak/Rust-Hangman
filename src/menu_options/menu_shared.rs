use super::main_menu_screen::main_menu;

pub fn clear_terminal() {
  print!("{}[2J", 27 as char);
}

pub fn go_to_main_menu() {
  let _ = main_menu();
}