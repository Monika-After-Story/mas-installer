use fltk::{
    app::{
        App,
        Sender
    },
    button::{
        // ButtonType,
        Button,
        CheckButton
    },
    draw,
    enums::{
        Align,
        Color,
        Event,
        FrameType,
        // FrameType,
        // LabelType
    },
    frame::Frame,
    group::{
        Pack,
        PackType
    },
    text::{
        TextBuffer,
        TextDisplay,
        WrapMode
    },
    prelude::{
        WidgetExt,
        WindowExt,
        GroupExt,
        WidgetBase,
        // ButtonExt,
        DisplayExt, ButtonExt
    },
    window::{
        Window,
        DoubleWindow
    }
};

use crate::{
    app_styles::*,
    // utils::*,
    Message,
    APP_LICENSE
};


/// Builds a default app
pub fn build_app() -> App {
    return App::default()
}


/// Builds an outer window
/// This is the main window of the app
/// Other windows get included into this
pub fn build_outer_win() -> DoubleWindow {
    let mut main_win = Window::default()
        .with_size(WIN_WIDTH, WIN_HEIGHT)
        .with_label(WIN_TITLE)
        .center_screen();
    main_win.set_color(C_DDLC_PINK_IDLE);

    main_win.end();

    return main_win
}


/// Builds an inner window
/// Inner windows are included into the main window. User can switch between them
pub fn build_inner_win() -> DoubleWindow {
    let mut inner_win = Window::default()
        .with_size(WIN_WIDTH - WIN_PADDING*2, WIN_HEIGHT - WIN_PADDING*2)
        .with_pos(WIN_PADDING, WIN_PADDING);
    inner_win.set_color(C_DDLC_WHITE_IDLE);

    inner_win.end();
    inner_win.hide();

    return inner_win
}


/// Callback for handling buttons events.
/// Allows to handle hover events
/// THIS IS GENERIC VERSION
fn __handle_button_widget(b: &mut dyn WidgetExt, ev: Event) -> bool {
    return match ev {
        Event::Enter | Event::Focus => {
            b.visible_focus(true);
            b.redraw();
            true
        },
        Event::Leave => {
            b.visible_focus(false);
            b.redraw();
            true
        },
        Event::Hide => {
            b.visible_focus(false);
            // don't want to mark this as handled
            false
        },
        // For all unhandled events we must return false
        _ => false
    }
}

/// Callback for handling buttons events.
/// Allows to handle hover events
/// This is a Button version
fn _handle_button(b: &mut Button, ev: Event) -> bool {
    return __handle_button_widget(b, ev)
}

/// Callback for handling how the button is being drawn.
/// Allows to style it
/// THIS IS GENERIC VERSION
fn __draw_button_widget(b: &mut dyn WidgetExt) {
    let (b_x, b_y, b_w, b_h) = (b.x(), b.y(), b.w(), b.h());

    let (frame_color, bg_color, text_color) = match b.has_visible_focus() {
        true => (C_DDLC_PINK_ACT, C_DDLC_WHITE_ACT, C_DDLC_PEACH),
        false => (C_DDLC_PINK_IDLE, C_DDLC_WHITE_IDLE, C_BLACK)
    };

    draw::draw_rect_fill(b_x, b_y, b_w, b_h, frame_color);
    draw::draw_rect_fill(b_x+BUT_PADDING, b_y+BUT_PADDING, b_w-BUT_PADDING*2, b_h-BUT_PADDING*2, bg_color);
    draw::set_draw_color(text_color);// for the text
    draw::set_font(BUT_FONT, BUT_FONT_SIZE);
    draw::draw_text2(&b.label(), b_x, b_y, b_w, b_h, b.align());
    b.redraw();
}

/// Callback for handling how the button is being drawn.
/// This is a Button version
fn _draw_button(b: &mut Button) {
    __draw_button_widget(b);
}

fn _build_button(width: i32, height: i32, label: &str, sender: Sender<Message>, msg: Message) -> Button {
    let mut but = Button::default()
        .with_size(width, height)
        .with_label(label);

    but.visible_focus(false);
    but.emit(sender, msg);
    but.handle(_handle_button);
    but.draw(_draw_button);

    return but
}

/// Builds a button with the given label, sender, and msg
/// The button won't be automatically position
pub fn build_button(label: &str, sender: Sender<Message>, msg: Message) -> Button {
    let but = _build_button(
        BUT_WIDTH,
        BUT_HEIGHT,
        label,
        sender,
        msg
    );
    return but
}

/// Builds a select directory dialogue button
pub fn build_sel_dir_button(label: &str, sender: Sender<Message>, msg: Message) -> Button {
    return _build_button(
        BUT_SEL_DIR_WIDTH,
        BUT_SEL_DIR_HEIGHT,
        label,
        sender,
        msg
    )
}


/// Callback for handling buttons events.
/// Allows to handle hover events
/// This is a CheckButton version
fn _handle_check_button(b: &mut CheckButton, ev: Event) -> bool {
    return __handle_button_widget(b, ev)
}

/// Callback for handling how the button is being drawn.
/// This is a version for Button
fn _draw_check_button(b: &mut CheckButton) {
    let (b_x, b_y, b_w, b_h) = (b.x(), b.y(), b.w(), b.h());

    let bg_color: Color;
    let txt_color: Color;
    if b.is_checked() {
        bg_color = C_DDLC_PINK_DARK;
        txt_color = C_DDLC_PEACH;
    }
    else if b.has_visible_focus() {
        bg_color = C_DDLC_WHITE_ACT;
        txt_color = C_BLACK;
    }
    else {
        bg_color = C_DDLC_WHITE_IDLE;
        txt_color = C_BLACK;
    }

    draw::draw_rect_fill(b_x, b_y, b_w, b_h, bg_color);

    let pad_outer = 3;
    draw::draw_rect_with_color(b_x+pad_outer, b_y+pad_outer, b_h-pad_outer*2, b_h-pad_outer*2, C_BLACK);
    if b.is_checked() {
        let pad_inner = pad_outer + 3;
        draw::draw_rect_fill(b_x+pad_inner, b_y+pad_inner, b_h-pad_inner*2, b_h-pad_inner*2, C_DDLC_PEACH);
        draw::draw_rect_with_color(b_x+pad_inner, b_y+pad_inner, b_h-pad_inner*2, b_h-pad_inner*2, C_BLACK);
    }

    draw::set_draw_color(txt_color);
    draw::set_font(BUT_FONT, BUT_FONT_SIZE);
    draw::draw_text2(&b.label(), b_x+b_h, b_y, b_w, b_h, b.align());

    b.redraw();
}

/// Builds a check button with the given parameters
fn _build_check_button(width: i32, height: i32, label: &str, sender: Sender<Message>, msg: Message) -> CheckButton {
    let mut but = CheckButton::default()
        .with_size(width, height)
        .with_label(label);

    but.visible_focus(false);
    but.emit(sender, msg);
    but.handle(_handle_check_button);
    but.draw(_draw_check_button);
    but.set_frame(FrameType::NoBox);
    but.set_down_frame(FrameType::NoBox);

    return but
}

// pub fn build_check_button(label: &str, sender: Sender<Message>, msg: Message) -> CheckButton {
//     let but = _build_check_button(
//         BUT_WIDTH,
//         BUT_HEIGHT,
//         label,
//         sender,
//         msg
//     );

//     return but
// }


/// Builds a frame at the given position
fn _build_frame(xpos: i32, ypos: i32) -> Frame {
    let frame = Frame::default()
        .with_size(WIN_WIDTH-WIN_PADDING*2-xpos, WIN_HEIGHT-WIN_PADDING*2-ypos)
        .with_pos(xpos, ypos);
        // .with_align(Align::Top | Align::Inside)
        // .with_label(label);

    return frame
}

/// Builds a frame at the top with the given label
fn _build_top_frame(label: &str) -> Frame {
    let mut frame = _build_frame(TOP_FRAME_XPOS, TOP_FRAME_YPOS);
    // frame.set_frame(FrameType::FlatBox);
    // frame.set_color(C_BLACK);
    frame.set_align(Align::Top | Align::Inside);
    frame.set_label(label);
    frame.set_label_color(C_DDLC_PINK_DARK);
    // frame.set_label_type(LabelType::Normal);
    frame.set_label_size(TOP_FRAME_LABEL_SIZE);

    return frame
}


fn _build_welcome_win_pack() -> Pack {
    const TOTAL_ITEMS: i32 = 2;
    const PAD_X: i32 = 50;
    let pad_y: i32 = WIN_HEIGHT-WIN_PADDING-BUT_HEIGHT-25;//mul_int_float(WIN_HEIGHT-WIN_PADDING-BUT_HEIGHT, 0.9);

    let mut pack = Pack::default()
        .with_size(WIN_WIDTH-WIN_PADDING*2-PAD_X*2, BUT_HEIGHT)
        .with_pos(PAD_X, pad_y)
        .with_align(Align::Center)
        .with_type(PackType::Horizontal);
    pack.set_spacing(WIN_WIDTH-WIN_PADDING*2 - BUT_WIDTH*TOTAL_ITEMS - PAD_X*2);

    pack.end();

    return pack
}

/// Builds the welcome windows
pub fn build_welcome_win(sender: Sender<Message>) -> DoubleWindow {
    let mut welcome_win = build_inner_win();
    welcome_win.show();
    welcome_win.begin();

    _build_top_frame(WELCOME_FRAME_LABEL);

    let welcome_but_pack = _build_welcome_win_pack();
    welcome_but_pack.begin();

    build_button(BUT_ABORT_LABEL, sender, Message::Close);
    build_button(BUT_CONTINUE_LABEL, sender, Message::NextPage);

    welcome_but_pack.end();

    welcome_win.end();

    return welcome_win
}


fn _build_license_win_inner_pack() -> Pack {
    const TOTAL_ITEMS: i32 = 2;

    let mut inner_pack = Pack::default()
        .with_size(BUT_WIDTH*TOTAL_ITEMS + 5, BUT_HEIGHT)
        .with_align(Align::Center)
        .with_type(PackType::Horizontal);
    inner_pack.set_spacing(5);

    inner_pack.end();

    return inner_pack
}

fn _build_license_win_outer_pack(spacing: i32) -> Pack {
    const PAD_X: i32 = 50;
    let pad_y: i32 = WIN_HEIGHT-WIN_PADDING-BUT_HEIGHT-25;//mul_int_float(WIN_HEIGHT-WIN_PADDING-BUT_HEIGHT, 0.9);

    let mut pack = Pack::default()
        .with_size(WIN_WIDTH-WIN_PADDING*2-PAD_X*2, BUT_HEIGHT)
        .with_pos(PAD_X, pad_y)
        .with_align(Align::Center)
        .with_type(PackType::Horizontal);
    pack.set_spacing(WIN_WIDTH-WIN_PADDING*2 - BUT_WIDTH - spacing - PAD_X*2);

    pack.end();

    return pack
}

fn _build_ternary_but_pack(sender: Sender<Message>) {
    let inner_pack = _build_license_win_inner_pack();

    inner_pack.begin();
    build_button(BUT_BACK_LABEL, sender, Message::PrevPage);
    build_button(BUT_CONTINUE_LABEL, sender, Message::NextPage);
    inner_pack.end();

    let mut outer_pack = _build_license_win_outer_pack(inner_pack.w());

    outer_pack.begin();
    build_button(BUT_ABORT_LABEL, sender, Message::Close);
    outer_pack.add(&inner_pack);
    outer_pack.end();
}

/// Builds the license window
pub fn build_license_win(sender: Sender<Message>) -> DoubleWindow {
    let license_win = build_inner_win();
    license_win.begin();


    _build_top_frame(LICENSE_FRAME_LABEL);

    let mut buf = TextBuffer::default();
    buf.set_text(APP_LICENSE);

    let mut txt = TextDisplay::default()
        .with_size(WIN_WIDTH-WIN_PADDING*2, 310)
        .with_pos(0, 100);
    txt.set_buffer(buf);

    _build_ternary_but_pack(sender);


    license_win.end();

    return license_win
}


/// Builds the select directory window
pub fn build_select_dir_win(sender: Sender<Message>, txt_buf: TextBuffer) -> DoubleWindow {
    let select_dir_win = build_inner_win();
    select_dir_win.begin();


    _build_top_frame(SELECT_DIR_FRAME_LABEL);

    let mut pack = Pack::default()
        .with_size(WIN_WIDTH-WIN_PADDING*2, SEL_DIR_TXT_HEIGHT)
        .with_align(Align::Center)
        .with_type(PackType::Horizontal);
    pack = pack.center_of_parent();
    pack.begin();

    let mut txt = TextDisplay::default()
        .with_size(WIN_WIDTH-WIN_PADDING*2-BUT_SEL_DIR_WIDTH, SEL_DIR_TXT_HEIGHT);
    txt.set_text_size(SEL_DIR_TXT_SIZE);
    txt.wrap_mode(WrapMode::None, 0);
    txt.set_buffer(txt_buf);

    build_sel_dir_button(BUT_SELECT_DIR_LABEL, sender, Message::SelectDir);

    pack.end();

    _build_ternary_but_pack(sender);


    select_dir_win.end();

    return select_dir_win
}


/// Builds options windows with various settings for installer
pub fn build_options_win(sender: Sender<Message>) -> DoubleWindow {
    let options_win = build_inner_win();
    options_win.begin();


    _build_top_frame(OPTIONS_FRAME_LABEL);


    const BUT_INST_SPR_WIDTH: i32 = BUT_WIDTH+45;
    const TOTAL_BUTS: i32 = 1;
    const XPOS: i32 = (WIN_WIDTH-WIN_PADDING*2)/2 - BUT_INST_SPR_WIDTH/2;
    const YPOS: i32 = (WIN_HEIGHT-WIN_PADDING*2)/2 - TOTAL_BUTS*BUT_HEIGHT/2;

    let mut inst_spr_but = _build_check_button(BUT_INST_SPR_WIDTH, BUT_HEIGHT, BUT_INSTALL_SPRITEPACKS_LABEL, sender, Message::InstallSpritepacks);
    inst_spr_but.set_checked(true);
    inst_spr_but.set_pos(XPOS, YPOS);


    _build_ternary_but_pack(sender);


    options_win.end();

    return options_win
}
