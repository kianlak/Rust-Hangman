mod structs;
mod data_access;
mod constants;
mod menu_options;
mod game;

extern crate serde;
extern crate serde_json;

use menu_options::menu_shared::go_to_main_menu;

fn main() {
  go_to_main_menu();
}