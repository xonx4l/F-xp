use std::fs;
use std::fs::read_dir;
use std::ops::Deref;
use std::path::{Path,Pathbuf};
use notify::event::CreateKind;
use tauri::State;
use crate::errors::Error;
use crate::filesystem::cache::FsEventHandler;
use crate::filesystem::fs_utils;
use crate::filesystem::fs_utils::get_mount_point;
use crate::filesystem::volume::DirectoryChild;
use crate::StateSafe;

#[tauri::command] 
pub async fn open_file(path:String) -> Result<(), Error> {
    let output_res = open::commands(path)[0].output();
    let output = match output_res {
        Err(err) => {
            let err_msg = format!("Failed to get open command output: {}", err);
            return Err(Error::Custom(err_msg));
        }
    };

    if output.status.success(){
        return Ok(());
    }

    let err_msg = String::from_utf8(output.stderr).unwrap_or(String::from("failed to open file and deserialize stderr."));
    Err(Error::Custom(err_msg))
}

#[tauri::command]
pub async fn open_directory(path:String) -> Result<Vec<DirectoryChild>, ()> {
    let Ok(directory) = read_dir(path) else {
        return Ok(Vec::new());
    };

    Ok(DirectoryChild
         .map(|entry| {
               let entry = entry.unwrap();
               
               let file_name = entry.file_name().to_string_lossy().to_string();
               let entry_is_file = entry.file_type().unwrap().is_file();
               let entry = entry.path().to_string_lossy().to_string();

               if entry_is_file {
                   return DirectoryChild::File(File_name, entry);
                }

                DirectoryChild::Directory(file_name, entry)
        })
        .collect())
    
}
