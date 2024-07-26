// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
fn main() {

    #[derive(Serialize, Deserialize)]
    pub struct CachedPath {
       #[serde(rename = "p")] 
        file_path:String,
        #[serde(rename = "t")]
        file_type: String,
    }

    pub tupe VolumeCache = Hashmap<String, Vec<CachedPath>>;

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
