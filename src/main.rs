#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
/// Program entry point

mod app;
mod audio;
mod errors;
mod installer;
mod static_data;
mod utils;


use std::collections::HashMap;
use regex::Regex;


// Get version from the cargo
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const DEF_VERSION: &str = "unknown build";

// GH link parts to accept the API
const ORG_NAME: &str = "Monika-After-Story";
const REPO_NAME: &str = "MonikaModDev";


// The map of regex patterns for the release assets
lazy_static::lazy_static! {
    pub static ref ASSETS_NAMES_RE_MAP: HashMap<&'static str, Regex> = {
        let mut hm = HashMap::new();
        hm.insert("def_ver", Regex::new(r"^Monika_After_Story-\d+\.\d+\.\d+-Mod\.zip$").unwrap());
        hm.insert("dlx_ver", Regex::new(r"^Monika_After_Story-\d+\.\d+\.\d+-Mod-Dlx\.zip$").unwrap());
        hm.insert("spr", Regex::new(r"^spritepacks\.zip$").unwrap());
        hm
    };
}


/// The entry point
fn main() {
    // This needs to be done first
    utils::disable_global_hotkeys();
    // Builds the app
    let mut app = app::InstallerApp::default();
    // Show it
    app.show();
    // Process events
    app.wait();
    // Explicitly drop
    drop(app);
}
