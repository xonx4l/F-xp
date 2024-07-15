// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler!)
               get_volumes,
               open_directory,
               search_directory,
               open_file,
               create_file,
               create_directory,
               rename_file,
               delete_file,
        ])
        .manage(Arc::new(Mutex::mew(AppState::default())))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
        
}
