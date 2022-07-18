// #![allow(dead_code, unused_imports, unused_mut, unused_variables)]// TODO: remove me
mod app_styles;
mod builder;
mod utils;


// Include the icon
static APP_ICON_DATA: &'static [u8] = include_bytes!("static/icon.png");


use fltk::{
    app::{
        channel,
        Sender,
        Receiver,
        // Scheme
    },
    enums::{
        Align,
        LabelType
    },
    frame::Frame,
    group::{
        Pack,
        PackType
    },
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
    use app_styles::*;


    utils::disable_global_hotkeys();

    let app = builder::build_app();
    let (sender, receiver): (Sender<Message>, Receiver<Message>) = channel();


    let mut main_win = builder::build_main_win();
    utils::load_icon(&mut main_win);
    main_win.begin();


    let welcome_win = builder::build_inner_win();
    welcome_win.begin();

    let frame_pos_x = 0;
    let frame_pos_y = utils::mul_int_float(WIN_HEIGHT, 0.1);
    let mut welcome_txt_frame = Frame::default()
        .with_size(WIN_WIDTH-WIN_PADDING*2-frame_pos_x, 100)
        .with_pos(frame_pos_x, frame_pos_y)
        // .with_align(Align::Inside)
        .with_label(WELCOME_FRAME_LABEL);

    welcome_txt_frame.set_label_color(C_DDLC_PINK_DARK);
    welcome_txt_frame.set_label_type(LabelType::Normal);
    welcome_txt_frame.set_label_size(28);


    let pack_pad_x = 50;
    let pack_pos_y = utils::mul_int_float(WIN_HEIGHT-BUT_HEIGHT, 0.9);
    let mut welcome_but_pack = Pack::default()
        .with_size(WIN_WIDTH-WIN_PADDING*2-pack_pad_x*2, BUT_HEIGHT)
        .with_pos(pack_pad_x, pack_pos_y)
        .with_align(Align::Center)
        .with_type(PackType::Horizontal);
    welcome_but_pack.set_spacing(WIN_WIDTH-WIN_PADDING*2 - BUT_WIDTH*2 - pack_pad_x*2);


    let _abort_button = builder::build_button("Abort", sender, Message::Close);

    let _continue_button = builder::build_button("Continue@>", sender, Message::NextPage);


    welcome_but_pack.end();
    welcome_win.end();


    main_win.end();

    let mut current_win_id: usize = 0;
    let mut windows: Vec<DoubleWindow> = vec![
        welcome_win
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
                    println!("Opened next page");
                },
                Message::PrevPage => {
                    let new_id = current_win_id-1;
                    utils::switch_win(&mut windows, &mut current_win_id, new_id);
                    println!("Opened previous page");
                },
            }
        }
    }
}
