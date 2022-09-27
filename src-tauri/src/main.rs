#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
mod service;

use service::commands::*;

fn main() {
  service::initiate_db();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      register_user, // register new user
      authenticate_user, // authenticate user
      reset_password, // reset the password of a user
      user_check_data, // check if user data is valid
      user_get_data, // get user data
      user_get_users, // get all users
      user_update_position, // update user position
      user_update_status, // update user status
      user_update_discord, // update user discord
      user_update_image, // update user image
      user_update, // update all user data
      position_check_position, // check if position is valid
      position_get_position, // get position data
      position_get_all_positions, // get all positions
      position_new, // create new position
      position_update, // update position


      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
