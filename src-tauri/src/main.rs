// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::Manager;

mod api;
mod command;
mod settings;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let auth = settings::Auth::init(&handle);

            if !auth.is_valid() {
                // Redirect to the login page if the user auth are unset
                let main_window = app.get_window("main").unwrap();
                let url = main_window.url().to_string();
                main_window
                    .eval(&format!("window.location.replace('{}')", url + "login"))
                    .expect("Unable to set window location");
            }

            app.manage(Mutex::new(auth));
            app.manage(api::ApiClient::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![command::login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
