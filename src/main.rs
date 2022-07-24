// #![allow(dead_code, unused_imports, unused_mut, unused_variables)]// TODO: remove me
mod app_styles;
mod builder;
mod utils;


// Include the icon
#[cfg(feature="compile_icon")]
static APP_ICON_DATA: &'static [u8] = include_bytes!("static/icon.png");
#[cfg(not(feature="compile_icon"))]
static APP_ICON_DATA: &'static [u8] = b"";
// Include license
#[cfg(feature="compile_license")]
static APP_LICENSE: &'static str = include_str!("static/license.md");
#[cfg(not(feature="compile_license"))]
static APP_LICENSE: &'static str = "Hello, World!";


// use std::path::PathBuf;

use fltk::{
    app::{
        channel,
        Sender,
        Receiver,
        // Scheme
    },
    // enums::{
    //     LabelType
    // },
    text::TextBuffer,
    prelude::{
        WidgetExt,
        GroupExt,
    },
    window::{
        // Window,
        DoubleWindow
    }
};


#[derive(Clone, Copy)]
pub enum Message {
    Close,
    NextPage,
    PrevPage,
    SelectDir,
    InstallSpritepacks
}


/// The entry point
fn main() {
    utils::disable_global_hotkeys();

    let (sender, receiver): (Sender<Message>, Receiver<Message>) = channel();
    let mut is_deluxe_version: bool = true;
    let mut extraction_dir = utils::get_cwd();
    let mut path_txt_buf = TextBuffer::default();
    path_txt_buf.set_text(extraction_dir.to_str().unwrap_or_default());

    let app = builder::build_app();

    let mut main_win = builder::build_outer_win();
    utils::load_icon(&mut main_win);
    main_win.begin();


    let welcome_win = builder::build_welcome_win(sender);
    let license_win = builder::build_license_win(sender);
    let dir_sel_win = builder::build_select_dir_win(sender, path_txt_buf.clone());
    let options_win = builder::build_options_win(sender);


    main_win.end();

    let mut current_win_id: usize = 0;
    let mut windows: Vec<DoubleWindow> = vec![
        welcome_win,
        license_win,
        dir_sel_win,
        options_win
    ];

    main_win.show();

    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                Message::Close => app.quit(),
                Message::NextPage => {
                    let new_id = current_win_id+1;
                    utils::switch_win(&mut windows, &mut current_win_id, new_id);
                },
                Message::PrevPage => {
                    let new_id = current_win_id-1;
                    utils::switch_win(&mut windows, &mut current_win_id, new_id);
                },
                Message::SelectDir => {
                    extraction_dir = utils::run_select_dir_dlg(app_styles::SEL_DIR_DLG_PROMPT);
                    path_txt_buf.set_text(extraction_dir.to_str().unwrap_or_default());
                }
                Message::InstallSpritepacks => {
                    is_deluxe_version = !is_deluxe_version;
                    println!("is deluxe: {:?}", is_deluxe_version);
                }
            }
        }
    }
}
