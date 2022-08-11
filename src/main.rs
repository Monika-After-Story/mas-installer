#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #![allow(dead_code, unused_imports, unused_mut, unused_variables)]// TODO: remove me
mod styles;
mod builder;
mod errors;
mod utils;
mod static_data;
mod audio;


use std::{
    thread,
    sync::{
        Arc,
        atomic::AtomicBool
    }
};

use fltk::{
    app::{
        channel,
        Sender,
        Receiver
    },
    text::TextBuffer,
    prelude::{
        WidgetExt,
        GroupExt,
    },
    window::DoubleWindow
};

use errors::InstallerError;


// Get version from the cargo
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const DEF_VERSION: &str = "unknown build";

// GH link parts to accept the API
const ORG_NAME: &str = "Monika-After-Story";
const REPO_NAME: &str = "MonikaModDev";

// IDs of assets in github release
const DEF_VERSION_ASSET_ID: usize = 1;
const DLX_VERSION_ASSET_ID: usize = 0;


#[derive(Clone, Copy)]
pub enum Message {
    UpdateProgressBar(f64),
    Close,
    NextPage,
    PrevPage,
    SelectDir,
    DlxVersionCheck,
    VolumeCheck,
    Install,
    Preparing,
    Downloading,
    Extracting,
    CleaningUp,
    Error,
    Abort,
    Done
}

type InstallResult = Result<(), InstallerError>;


/// The entry point
fn main() {
    // This needs to be done asap
    utils::disable_global_hotkeys();

    let audio_manager = match audio::play_theme() {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Failed to init audio: {e}");
            None
        }
    };

    // Some things our program will use
    let (sender, receiver): (Sender<Message>, Receiver<Message>) = channel();
    let abort_flag = Arc::new(AtomicBool::new(false));
    let mut installer_th_handle: Option<thread::JoinHandle<InstallResult>> = None;
    let mut is_deluxe_version: bool = true;
    let mut extraction_dir = utils::get_cwd();
    let mut path_txt_buf = TextBuffer::default();
    path_txt_buf.set_text(extraction_dir.to_str().unwrap_or_default());
    let mut progress_bar = builder::build_progress_bar();

    let app = builder::build_app();

    let mut main_win = builder::build_outer_win(sender, &abort_flag);
    main_win.begin();


    let welcome_win = builder::build_welcome_win(sender);
    let license_win = builder::build_license_win(sender);
    let dir_sel_win = builder::build_select_dir_win(sender, path_txt_buf.clone());
    let options_win = builder::build_options_win(sender, is_deluxe_version);
    let progress_win = builder::build_propgress_win(sender, &progress_bar);
    let mut abort_win = builder::build_abort_win(sender);
    let mut done_win = builder::build_done_win(sender);


    main_win.end();

    let mut current_win_id: usize = 0;
    let mut windows: Vec<DoubleWindow> = vec![
        welcome_win,
        license_win,
        dir_sel_win,
        options_win,
        progress_win
    ];

    main_win.show();

    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                Message::UpdateProgressBar(val) => {
                    progress_bar.set_value(val);
                },
                Message::Close => {
                    break;
                },
                Message::NextPage => {
                    utils::show_next_win(&mut windows, &mut current_win_id);
                },
                Message::PrevPage => {
                    utils::show_previous_win(&mut windows, &mut current_win_id);
                },
                Message::SelectDir => {
                    let selected_dir = utils::run_select_dir_dlg(styles::SEL_DIR_DLG_PROMPT);
                    if !utils::is_valid_ddlc_dir(&selected_dir) {
                        utils::run_msg_dlg("Attention!\nSelected directory doesn't appear to be\na valid DDLC directory");
                    }
                    if selected_dir.is_dir() && selected_dir.parent().is_some() {
                        extraction_dir = selected_dir;
                        path_txt_buf.set_text(extraction_dir.to_str().unwrap_or_default());
                    }
                },
                Message::DlxVersionCheck => {
                    is_deluxe_version = !is_deluxe_version;
                    match is_deluxe_version {
                        true => println!("Using deluxe version..."),
                        false => println!("Using standard version...")
                    };
                },
                Message::VolumeCheck => {
                    if let Some(ref am) = audio_manager {
                        let sink = am.get_sink();
                        if sink.volume() == 0.0{
                            sink.set_volume(1.0);
                            println!("Audio unmuted...")
                        }
                        else {
                            sink.set_volume(0.0);
                            println!("Audio muted...")
                        }
                    }
                }
                Message::Install => {
                    // We warn the user again if the extraction dir looks wrong
                    if !utils::is_valid_ddlc_dir(&extraction_dir) {
                        utils::run_msg_dlg("Attention!\nInstalling into a non-DDLC directory");
                    }
                    // We also need to move to the next window
                    sender.send(Message::NextPage);
                    // Consume any existing thread first
                    cleanup_th_handle(installer_th_handle);
                    // Start a new thread
                    installer_th_handle = Some(
                        utils::install_game_in_thread(&extraction_dir, sender, &abort_flag, is_deluxe_version)
                    );
                },
                Message::Preparing => {
                    println!("Preparing...");
                    progress_bar.set_label("Preparing...");
                },
                Message::Downloading => {
                    println!("Done!\nDownloading...");
                    progress_bar.set_label("Downloading...");
                },
                Message::Extracting => {
                    println!("Done!\nExtracting...");
                    progress_bar.set_label("Extracting...");
                },
                Message::CleaningUp => {
                    println!("Done!\nCleaning up...");
                    progress_bar.set_label("Cleaning up...");
                },
                Message::Error => {
                    println!("An error has occurred...");
                    utils::set_flag(&abort_flag, true);
                    let rv = cleanup_th_handle(installer_th_handle);
                    // We've moved the handle, set it to None
                    installer_th_handle = None;
                    // Show the error if we can
                    if let Some(e) = rv {
                        utils::run_alert_dlg(&format!("{}", e));
                    }
                    // Let's just quit
                    sender.send(Message::Close);
                },
                Message::Abort => {
                    println!("Installation has been aborted!");
                    utils::set_flag(&abort_flag, true);
                    cleanup_th_handle(installer_th_handle);
                    installer_th_handle = None;
                    utils::hide_current_win(&mut windows, current_win_id);
                    abort_win.show();
                },
                Message::Done => {
                    println!("Done!\nInstallation is complete!");
                    utils::hide_current_win(&mut windows, current_win_id);
                    done_win.show();
                }
            };
        }
    }
    cleanup_th_handle(installer_th_handle);
    app.quit();
    if audio_manager.is_some() {
        audio_manager.unwrap().stop();
    }
}

/// Joins the thread handle
fn cleanup_th_handle(th_handle: Option<thread::JoinHandle<InstallResult>>) -> Option<InstallerError> {
    if let Some(th_handle) = th_handle {
        match th_handle.join() {
            Err(rv) => {
                eprintln!("Failed to join installer thread {:?}", rv);
            },
            Ok(rv) => {
                if let Err(e) = rv {
                    eprintln!("Installer thread failed: {}", e);
                    return Some(e);
                }
            }
        };
    }
    return None;
}
