/// The module that implements AppState

use std::{
    path::PathBuf,
    sync::{
        Arc,
        Mutex
    },
};
use crate::audio::Volume;


pub type ThreadSafeState = Arc<Mutex<AppState>>;


/// Struct representing app state
#[derive(Debug)]
pub struct AppState {
    extraction_dir: PathBuf,
    abort_flag: bool,
    deluxe_ver_flag: bool,
    install_spr_flag: bool,
    music_volume: Volume
}

impl AppState {
    /// Creates a new AppState
    pub fn new(
        extraction_dir: PathBuf,
        abort_flag: bool,
        deluxe_ver_flag: bool,
        install_spr_flag: bool,
        music_volume: Volume
    ) -> Self {
        return Self {
            extraction_dir,
            abort_flag,
            deluxe_ver_flag,
            install_spr_flag,
            music_volume
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

    /// Returns the install spritepacks flag
    pub fn get_install_spr_flag(&self) -> bool {
        return self.install_spr_flag;
    }

    /// Sets the install spritepacks flag
    #[allow(dead_code)]
    pub fn set_install_spr_flag(&mut self, value: bool) {
        self.install_spr_flag = value;
    }

    /// Inverts the install spritepacks flag
    pub fn invert_install_spr_flag(&mut self) {
        self.install_spr_flag = !self.install_spr_flag;
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

    /// Returns the abort flag
    pub fn get_music_volume(&self) -> Volume {
        return self.music_volume;
    }

    /// Sets the abort flag
    pub fn set_music_volume(&mut self, value: Volume) {
        self.music_volume = value;
    }
}

impl Default for AppState {
    fn default() -> Self {
        return Self::new(
            crate::utils::get_cwd(),
            false,
            true,
            false,
            1.0
        );
    }
}


/// Builds an AppState and wraps it into a Mutex inside an Arc
pub fn build_thread_safe_state() -> ThreadSafeState {
    return Arc::new(Mutex::new(AppState::default()));
}
