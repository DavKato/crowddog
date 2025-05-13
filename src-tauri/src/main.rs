// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::Manager;

mod api;
mod command;
mod settings;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let client = api::ApiClient::new();
            app.manage(client);

            let cred = settings::Credentials::init(&handle);

            let mut is_logged_in = false;
            if cred.is_valid() {
                // Try to login with the saved cred
                if let Ok(()) =
                    tauri::async_runtime::block_on(app.state::<api::ApiClient>().login(&cred))
                {
                    is_logged_in = true;
                }
            }
            let main_window = app.get_window("main").unwrap();
            let mut url = main_window.url();
            url.set_path("/login");
            if is_logged_in {
                url.set_query(Some("logged_in=true"));
            }
            main_window
                .eval(&format!("window.location.replace('{}')", url.as_str()))
                .expect("Unable to set window location");

            app.manage(Mutex::new(cred));
            app.manage::<command::TimerHandle>(Mutex::new(None));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::login,
            command::re_login,
            command::init_data,
            command::get_my_patterns,
            command::get_processes,
            command::set_work_content,
            command::start_timer,
            command::stop_timer,
            command::cancel_timer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
