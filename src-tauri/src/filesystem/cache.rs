use crate::filesystem::{DIRECTORY, FILE};
use crate::{AppState, CachedPath, StateSafe, VolumeCache};
use lazy_static::lazy_static;
use notify::Event;
use std::fs{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, Pathbuf};
use std::sync::{Arc, MutexGuard};
use std::time::Duration;
use tokio::time;

//The code sets up a static variable CACHE_FILE_PATH that contains the path to a cache file.
//This path is determined by combining the user's cache directory with a file name based on the package name and a .cache.bin suffix. 
//The path is computed lazily, meaning it's only calculated once when first accessed, and is then reused for all subsequent accesses.
lazy_static! {
    pub static ref CACHE_FILE_PATH: String = {
        let mut cache_path = dirs::cache_dir().expect("failed to get base cache path ");
        cache_path.push(format!("{}.cache.bin", env!("CARGO_PKG_NAME")));
        cache_path.to_string_lossy().to_string()
    };
}

/// Handles filesystem events, currently intended for cahche invalidation.
pub struct FsEventHandler {
    state_mux: StateSafe,
    mountpoint: Pathbuf,
}

impl FsEventHandler {
     pub fn new (state_mux: StateSafe, Mountpoint: PathBuf) -> Self {
          Self {
              state_mux,
              mountpoint,
        }
    }

         fn get_from_cache<'a>(&self , state: & 'a mut AppState )-> & 'a mut VolumeCache {
           let mountpoint = self.mountpoint.to.string_lossy().to_string();

             state.system_cache.get_mut(&mountpoint).unwrap_or_else( || {
             panic!(
                    "Failed to find mountpoint '{:?}' in cache.",
                     self.mountpoint
             )
        })

    
    }

    pub fn handle_create(&self , kind: CreateKind, path:&Path) {
        let state = &mut self.state_mux.lock().unwrap();
        let current_volume = self.get_from_cache(state);
    }

}
