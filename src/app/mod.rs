#![allow(dead_code)]
/// The module that implements our app

pub mod state;


use std::thread;

use fltk::{
    app::{
        // channel,
        Sender,
        Receiver
    },
    text::TextBuffer,
    misc::Progress,
    window::DoubleWindow
};

use state::AppState;
use crate::{Message, InstallResult};
use super::audio;


/// A struct representing our app
pub struct App {
    // The app state
    state: AppState,

    // Communication managers
    sender: Sender<Message>,
    receiver: Receiver<Message>,

    // The main window of the app
    main_window: DoubleWindow,
    // The windows the user can switch
    // using the back & continue buttons
    linked_windows: [DoubleWindow; 5],
    // Current window id
    current_window_id: usize,
    // These windows need to be available directly
    abort_window: DoubleWindow,
    done_window: DoubleWindow,

    // Audio manager, option because audio might not work
    audio_manager: Option<audio::AudioManager>,

    // Handle to the installer thread, option because we might not start it/close early
    installer_th_handle: Option<thread::JoinHandle<InstallResult>>,

    // These need to be updated
    path_txt_buf: TextBuffer,
    progress_bar: Progress
}
