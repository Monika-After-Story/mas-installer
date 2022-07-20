// #![allow(dead_code, unused_imports, unused_mut, unused_variables)]// TODO: remove me
mod app_styles;
mod builder;
mod utils;


// Include the icon
static APP_ICON_DATA: &'static [u8] = include_bytes!("static/icon.png");
// Include license
static APP_LICENSE: &'static str = include_str!("static/license.md");


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
    PrevPage
}


/// The entry point
fn main() {
    // use app_styles::*;
    utils::disable_global_hotkeys();

    let app = builder::build_app();
    let (sender, receiver): (Sender<Message>, Receiver<Message>) = channel();


    let mut main_win = builder::build_outer_win();
    utils::load_icon(&mut main_win);
    main_win.begin();


    let welcome_win = builder::build_welcome_win(sender);

    let license_win = builder::build_license_win(sender);



    main_win.end();

    let mut current_win_id: usize = 0;
    let mut windows: Vec<DoubleWindow> = vec![
        welcome_win,
        license_win
    ];


    main_win.show();


    // match app.run() {
    //     Ok(_) => println!("Shuting down the app"),
    //     Err(err) => eprintln!("Crash during even loop: {}", err)
    // }
    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                Message::Close => app.quit(),
                Message::NextPage => {
                    let new_id = current_win_id+1;
                    utils::switch_win(&mut windows, &mut current_win_id, new_id);
                    // println!("Opened next page");
                },
                Message::PrevPage => {
                    let new_id = current_win_id-1;
                    utils::switch_win(&mut windows, &mut current_win_id, new_id);
                    // println!("Opened previous page");
                },
            }
        }
    }
}
