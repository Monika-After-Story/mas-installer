use std::{
    env,
    path::PathBuf
};

use fltk::{
    image,
    app::add_handler,
    dialog::{
        NativeFileChooser,
        NativeFileChooserType,
        // NativeFileChooserOptions
    },
    enums::Event,
    window::DoubleWindow,
    prelude::{
        WidgetExt,
        WindowExt
    },
};

use crate::APP_ICON_DATA;


/// Multiplies int by float and returns int
/// Useful to position widgets relatively of the windows size
// pub fn mul_int_float(a: i32, b: f32) -> i32 {
//     return (a as f32 * b) as i32
// }


/// Changes current active windows by hiding one window and showing another
pub fn switch_win(windows: &mut Vec<DoubleWindow>, current_id: &mut usize, new_id: usize) {
    // Sanity check
    if *current_id >= windows.len() || new_id >= windows.len() {
        return
    }
    windows[*current_id].hide();
    windows[new_id].show();
    *current_id = new_id;
}


/// Loads icon data and sets it as window icon
pub fn load_icon(win: &mut DoubleWindow) {
    let icon = image::PngImage::from_data(&APP_ICON_DATA);
    win.set_icon(icon.ok());
}

/// Disables global hotkeys by consuming all shortcut events
pub fn disable_global_hotkeys() {
    add_handler(
        |ev| {
            return match ev {
                Event::Shortcut => true,
                _ => false
            }
        }
    );
}

/// Returns current working dir
pub fn get_cwd() -> PathBuf {
    let cwd = env::current_dir();
    return cwd.ok().unwrap_or_default()
}

/// Launches select directory dialogue native to the target OS
/// returns selected directory, defaults to current working directory
pub fn run_select_dir_dlg(prompt: &str) -> PathBuf {
    let mut c = NativeFileChooser::new(NativeFileChooserType::BrowseDir);

    c.set_title(prompt);

    let cwd = get_cwd();
    match c.set_directory(&cwd) {
        Err(err) => eprintln!("Failed to automatically set default dir: {err}"),
        Ok(_) => {}
    }

    c.show();

    return c.filename()
}
