/// The module that implements various dialogs

use std::path::PathBuf;

use fltk::{
    app::wait,
    dialog::{
        NativeFileChooser,
        NativeFileChooserType
    },
    prelude::{
        WidgetExt,
        WindowExt
    },
};

use crate::utils;
use super::builder;


/// Launches select directory dialog native to the target OS
/// returns selected directory, defaults to current working directory
pub fn run_select_dir_dlg(prompt: &str) -> PathBuf {
    let mut c = NativeFileChooser::new(NativeFileChooserType::BrowseDir);

    c.set_title(prompt);

    let cwd = utils::get_cwd();
    match c.set_directory(&cwd) {
        Err(err) => eprintln!("Failed to automatically set default dir: {err}"),
        Ok(_) => {}
    };

    c.show();

    return c.filename();
}

/// Launches alert dialog
/// NOTE: modal
pub fn run_alert_dlg(msg: &str) {
    let mut win = builder::build_alert_win(
        msg
    );
    win.show();
    while win.shown() {
        wait();
    }
    drop(win);
}

/// Launches message dialog
/// NOTE: modal
pub fn run_msg_dlg(msg: &str) {
    let mut win = builder::build_msg_win(
        msg
    );
    win.show();
    while win.shown() {
        wait();
    }
    drop(win);
}
