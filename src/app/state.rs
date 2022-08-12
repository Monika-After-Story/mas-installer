/// The module that implements AppState

use std::{
    path::PathBuf,
    sync::{
        Arc,
        Mutex
    },
};


pub type ThreadSafeState = Arc<Mutex<AppState>>;


/// Struct representing app state
#[derive(Debug)]
pub struct AppState {
    abort_flag: bool,
    deluxe_ver_flag: bool,
    extraction_dir: PathBuf
}

impl AppState {
    /// Creates a new AppState
    pub fn new(abort_flag: bool, deluxe_ver_flag: bool, extraction_dir: PathBuf) -> Self {
        return Self {
            abort_flag,
            deluxe_ver_flag,
            extraction_dir
        };
    }

    /// Returns the abort flag
    pub fn get_abort_flag(&self) -> bool {
        return self.abort_flag;
    }

    /// Sets the abort flag
    pub fn set_abort_flag(&mut self, value: bool) {
        self.abort_flag = value;
    }

    /// Returns the dlx version flag
    pub fn get_deluxe_ver_flag(&self) -> bool {
        return self.deluxe_ver_flag;
    }

    /// Sets the dlx version flag
    #[allow(dead_code)]
    pub fn set_deluxe_ver_flag(&mut self, value: bool) {
        self.deluxe_ver_flag = value;
    }

    /// Inverts the dlx version flag
    pub fn invert_deluxe_ver_flag(&mut self) {
        self.deluxe_ver_flag = !self.deluxe_ver_flag;
    }

    /// Returns the extraction directory
    pub fn get_extraction_dir(&self) -> &PathBuf {
        return &self.extraction_dir;
    }

    /// Returns the extraction directory
    pub fn get_extraction_dir_str(&self) -> &str {
        return self.extraction_dir.to_str().unwrap_or_default();
    }

    /// Sets the extraction directory
    pub fn set_extraction_dir(&mut self, new_path: PathBuf) {
        self.extraction_dir = new_path;
    }
}

impl Default for AppState {
    fn default() -> Self {
        return Self::new(
            false,
            true,
            crate::utils::get_cwd()
        );
    }
}


/// Builds an AppState and wraps it into a Mutex inside an Arc
pub fn build_thread_safe_state() -> ThreadSafeState {
    return Arc::new(Mutex::new(AppState::default()));
}
