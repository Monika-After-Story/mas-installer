/// Module with utils functions

use std::{
    env,
    path::PathBuf,
    fs::read_dir
};

use fltk::{
    app::add_handler,
    enums::Event
};


/// Disables global hotkeys by consuming all shortcut events
pub fn disable_global_hotkeys() {
    add_handler(
        |ev| {
            return match ev {
                Event::Shortcut => true,
                _ => false
            };
        }
    );
}


/// Returns current working dir
pub fn get_cwd() -> PathBuf {
    let cwd = env::current_dir();
    return cwd.ok().unwrap_or_default();
}

/// Checks if the given path is a valid DDLC directory
pub fn is_valid_ddlc_dir(path: &PathBuf) -> bool {
    const TOTAL_CONDITIONS: u16 = 5;
    const REQUIRED_FLAG: u16 = 2 << TOTAL_CONDITIONS;

    if !path.exists() || !path.is_dir() {
        return false;
    }

    let content = read_dir(path);
    if content.is_err() {
        eprintln!("Failed to read content of the selected folder");
        // If we failed to read, we allow to install anyway - the folder might be valid
        return true;
    }

    let content = content.unwrap();
    let mut flag: u16 = 2;
    for item in content {
        if item.is_err() {
            eprintln!("Failed to read content of the selected folder");
            return true;
        }

        let item = item.unwrap();
        let file_name = item.file_name().into_string();
        // It should be valid utf-8, otherwise it's unlikely to be a DDLC file and we can skip
        if file_name.is_err() {
            continue;
        }
        let file_name = file_name.unwrap();
        let file_name_str = file_name.as_str();

        // Increase flag for each condition met
        // start from 2 and 5 conditions, means 2^6
        if item.path().is_dir() {
            match file_name_str {
                "characters" | "game" | "renpy" => {flag *= 2;},
                _ => {},
            };
        }
        else {
            match file_name_str {
                "DDLC.py" | "DDLC.sh" => {flag *= 2;},
                _ => {},
            };
        }

        if flag == REQUIRED_FLAG {
            return true;
        }
    }

    return flag == REQUIRED_FLAG;
}
