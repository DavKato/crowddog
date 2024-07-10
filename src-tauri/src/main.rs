// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        // .setup(|app| {
        //     let main_window = app.get_window("main").unwrap();
        //     let url = main_window.url().to_string();
        // Redirect to the first tab
        // main_window
        //     .eval(&format!(
        //         "window.location.replace('{}')",
        //         url + &tabs[0].id.to_string()
        //     ))
        //     .expect("Unable to set window location");
        // app.manage(store::InternalState::new(tabs));
        //     Ok(())
        // })
        // .invoke_handler(tauri::generate_handler![
        //     command::store::retrieve_state,
        // ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
