/// The module that implements our app

pub mod builder;
pub mod state;
pub mod styles;


use std::{thread, mem, path::PathBuf};

use fltk::{
    app::{
        App as fltkApp,
        channel,
        Sender,
        Receiver
    },
    text::TextBuffer,
    misc::Progress,
    prelude::{
        WidgetExt,
        GroupExt,
    },
    window::DoubleWindow
};

use state::{ThreadSafeState, build_thread_safe_state};
use crate::{Message, InstallResult};
use super::{audio, utils, errors};
use errors::InstallerError;


/// A struct representing our app
pub struct InstallerApp {
    inner: fltkApp,
    // The app state
    state: ThreadSafeState,

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

impl InstallerApp {
    /// Creates a new App, must be used at runtime
    pub fn new() -> Self {
        let (sender, receiver): (Sender<Message>, Receiver<Message>) = channel();

        let state = build_thread_safe_state();

        let path_txt_buf = TextBuffer::default();
        let progress_bar = builder::build_progress_bar();

        let main_window = builder::build_outer_win(sender, &state);
        main_window.begin();

        let linked_windows = {
            let s = state.lock().unwrap();
            let is_dlx_version = s.get_deluxe_ver_flag();
            let install_spr = s.get_install_spr_flag();
            [
                builder::build_welcome_win(sender),
                builder::build_license_win(sender),
                builder::build_select_dir_win(sender, path_txt_buf.clone()),
                builder::build_options_win(sender, is_dlx_version, install_spr),
                builder::build_propgress_win(sender, &progress_bar)
            ]
        };

        let abort_window = builder::build_abort_win(sender);
        let done_window = builder::build_done_win(sender);

        main_window.end();

        let audio_manager = match audio::play_theme() {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("Failed to init audio: {e}");
                None
            }
        };

        let mut installer = Self {
            inner: builder::build_app(),
            state,
            sender,
            receiver,
            main_window,
            linked_windows,
            current_window_id: 0,
            abort_window,
            done_window,
            audio_manager,
            installer_th_handle: None,
            path_txt_buf,
            progress_bar
        };
        // Imprortant to set the dir again to update the text disp buffer
        installer.set_extraction_dir(utils::get_cwd());

        return installer;
    }

    /// Shows the main window, thus renders the app
    pub fn show(&mut self) {
        self.main_window.show();
    }

    /// Blocks current thread and runs event loop of the app
    pub fn wait(&mut self) {
        while self.inner.wait() {
            if let Some(msg) = self.receiver.recv() {
                match msg {
                    Message::UpdateProgressBar(val) => {
                        self.progress_bar.set_value(val);
                    },
                    Message::Close => {
                        break;
                    },
                    Message::NextPage => {
                        self.show_next_window();
                    },
                    Message::PrevPage => {
                        self.show_previous_window();
                    },
                    Message::SelectDir => {
                        let selected_dir = utils::run_select_dir_dlg(styles::SEL_DIR_DLG_PROMPT);
                        if !utils::is_valid_ddlc_dir(&selected_dir) {
                            utils::run_msg_dlg("Attention!\nSelected directory doesn't appear to be\na valid DDLC directory");
                        }
                        self.set_extraction_dir(selected_dir);
                    },
                    Message::DlxVersionCheck => {
                        let mut app_state = self.state.lock().unwrap();
                        app_state.invert_deluxe_ver_flag();
                        match app_state.get_deluxe_ver_flag() {
                            true => println!("Using deluxe version..."),
                            false => println!("Using standard version...")
                        };
                    },
                    Message::InstallSprCheck => {
                        let mut app_state = self.state.lock().unwrap();
                        app_state.invert_install_spr_flag();
                        match app_state.get_install_spr_flag() {
                            true => println!("Including spritepacks..."),
                            false => println!("Excluding spritepacks...")
                        };
                    }
                    Message::VolumeCheck => {
                        if let Some(ref am) = self.audio_manager {
                            if am.get_volume() == 0.0{
                                am.set_volume(1.0);
                                println!("Audio unmuted...")
                            }
                            else {
                                am.set_volume(0.0);
                                println!("Audio muted...")
                            }
                        }
                    }
                    Message::Install => {
                        let app_state = self.state.lock().unwrap();
                        // We warn the user again if the extraction dir looks wrong
                        if !utils::is_valid_ddlc_dir(app_state.get_extraction_dir()) {
                            utils::run_msg_dlg("Attention!\nInstalling into a non-DDLC directory");
                        }
                        // We also need to move to the next window
                        self.sender.send(Message::NextPage);
                        // Consume any existing thread first
                        // Have to drop the old ref
                        drop(app_state);
                        self.cleanup_th_handle();
                        // Start a new thread
                        self.installer_th_handle = Some(
                            utils::install_game_in_thread(self.sender, &self.state)
                        );
                    },
                    Message::Preparing => {
                        println!("Preparing...");
                        self.progress_bar.set_label("Preparing...");
                    },
                    Message::Downloading => {
                        println!("Done!\nDownloading...");
                        self.progress_bar.set_label("Downloading...");
                    },
                    Message::Extracting => {
                        println!("Done!\nExtracting...");
                        self.progress_bar.set_label("Extracting...");
                    },
                    Message::DownloadingSpr => {
                        println!("Done!\nDownloading spritepacks...");
                        self.progress_bar.set_label("Downloading spritepacks...");
                    },
                    Message::ExtractingSpr => {
                        println!("Done!\nExtracting spritepacks...");
                        self.progress_bar.set_label("Extracting spritepacks...");
                    },
                    Message::CleaningUp => {
                        println!("Done!\nCleaning up...");
                        self.progress_bar.set_label("Cleaning up...");
                    },
                    Message::Error => {
                        println!("An error has occurred...");
                        self.abort_installation();
                        let rv = self.cleanup_th_handle();
                        // Show the error if we can
                        if let Some(e) = rv {
                            utils::run_alert_dlg(&format!("{e}"));
                        }
                        // Let's just quit
                        self.sender.send(Message::Close);
                    },
                    Message::Abort => {
                        println!("Installation has been aborted!");
                        self.abort_installation();
                        self.cleanup_th_handle();
                        self.hide_current_window();
                        self.abort_window.show();
                    },
                    Message::Done => {
                        println!("Done!\nInstallation is complete!");
                        self.abort_installation();
                        self.hide_current_window();
                        self.done_window.show();
                    }
                };
            }
        }
    }

    /// Changes current active windows by hiding one window and showing another
    pub fn change_window(&mut self, new_id: usize) {
        // Sanity check
        let max_id = self.linked_windows.len()-1;
        if new_id > max_id {
            return;
        }
        self.linked_windows[self.current_window_id].hide();
        self.linked_windows[new_id].show();
        self.current_window_id = new_id;
    }

    /// Hides current window
    pub fn hide_current_window(&mut self) {
        self.linked_windows[self.current_window_id].hide();
    }

    /// Shows current window
    #[allow(dead_code)]
    pub fn show_current_window(&mut self) {
        self.linked_windows[self.current_window_id].show();
    }

    /// Hides current and shows next (current id + 1) window
    pub fn show_next_window(&mut self) {
        self.change_window(self.current_window_id + 1);
    }

    /// Hides current and shows previous (current id - 1) window
    pub fn show_previous_window(&mut self) {
        // Overflow sanity check
        if self.current_window_id == 0 {
            return;
        }
        self.change_window(self.current_window_id - 1);
    }

    /// Joins the installer thread handle
    fn cleanup_th_handle(&mut self) -> Option<InstallerError> {
        // I couldn't find a way to join a thread behind a "mut self reference"
        // This *should* work, but it doesn't:
        //      self.installer_th_handle.unwrap().join();
        //      self.installer_th_handle = None;
        // We can do a trick tho: move the handle to a new variable and put None into the old variable using mem::replace
        // Technically that is what I'd do in the example above too,
        // but I guess the compiler can't understand it
        let th_handle = mem::replace(&mut self.installer_th_handle, None);

        if let Some(th_handle) = th_handle {
            match th_handle.join() {
                Ok(rv) => {
                    if let Err(e) = rv {
                        eprintln!("Installer thread failed: {}", e);
                        return Some(e);
                    }
                },
                Err(rv) => {
                    eprintln!("Failed to join installer thread {:?}", rv);
                }
            };
        }
        return None;
    }

    /// Aborts installation process by setting the flag
    /// NOTE: DOES NOT JOIN THE THREAD
    fn abort_installation(&mut self) {
        let mut app_state = self.state.lock().expect("Failed to lock app state mutex");
        app_state.set_abort_flag(true);
    }

    /// Updates the extraction dir and text display with the path
    fn set_extraction_dir(&mut self, new_dir: PathBuf) {
        if new_dir.is_dir() && new_dir.parent().is_some() {
            let mut app_state = self.state.lock().unwrap();
            app_state.set_extraction_dir(new_dir);
            self.path_txt_buf.set_text(app_state.get_extraction_dir_str());
        }
    }
}

impl Default for InstallerApp {
    fn default() -> Self {
        return Self::new();
    }
}

impl Drop for InstallerApp {
    fn drop(&mut self) {
        // Stop the installer thread
        self.abort_installation();
        self.cleanup_th_handle();
        // Stop the music, cleanup memory
        let am = mem::replace(&mut self.audio_manager, None);
        if let Some(am) = am {
            am.stop();
        }
        // Stop the app
        self.inner.quit();
    }
}
