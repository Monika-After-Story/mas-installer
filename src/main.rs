#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
/// Program entry point

mod app;
mod audio;
mod errors;
mod installer;
mod static_data;
mod utils;


// Get version from the cargo
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const DEF_VERSION: &str = "unknown build";

// GH link parts to accept the API
const ORG_NAME: &str = "Monika-After-Story";
const REPO_NAME: &str = "MonikaModDev";

// IDs of assets in github release
const DEF_VERSION_ASSET_ID: usize = 1;
const DLX_VERSION_ASSET_ID: usize = 0;
const SPR_ASSET_ID: usize = 3;


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
}
