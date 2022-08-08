use fltk::{
    app::{
        App,
        Sender,
        screen_size
    },
    button::{
        Button,
        CheckButton
    },
    draw,
    enums::{
        Align,
        Color,
        Event,
        FrameType
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
        DisplayExt,
        ButtonExt
    },
    misc::Progress,
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
    return App::default();
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

    return main_win;
}


/// Builds an inner window
/// Inner windows are included into the main window. User can switch between them
pub fn build_inner_win() -> DoubleWindow {
    let mut inner_win = Window::default()
        .with_size(INNER_WIN_WIDTH, INNER_WIN_HEIGHT)
        .with_pos(WIN_PADDING, WIN_PADDING);
    inner_win.set_color(C_DDLC_WHITE_IDLE);

    inner_win.end();
    inner_win.hide();

    return inner_win;
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
    };
}

/// Callback for handling buttons events.
/// Allows to handle hover events
/// This is a Button version
fn _handle_button(b: &mut Button, ev: Event) -> bool {
    return __handle_button_widget(b, ev);
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

fn _build_button_base<H, D>(width: i32, height: i32, label: &str, handler: H, draw: D) -> Button
where
    H: FnMut(&mut Button, Event) -> bool + 'static,
    D: FnMut(&mut Button) + 'static
{
    let mut but = Button::default()
        .with_size(width, height)
        .with_label(label);

    but.visible_focus(false);
    but.handle(handler);
    but.draw(draw);

    return but;
}

fn _build_button_adv(width: i32, height: i32, label: &str, sender: Sender<Message>, msg: Message) -> Button {
    let mut but = _build_button_base(
        width,
        height,
        label,
        _handle_button,
        _draw_button
    );
    but.emit(sender, msg);

    return but;
}

/// Builds a button with the given label, sender, and msg
/// The button won't be automatically position
/// width, height, ev handler, and draw func are pre-defined
pub fn build_button(label: &str, sender: Sender<Message>, msg: Message) -> Button {
    let but = _build_button_adv(
        BUT_WIDTH,
        BUT_HEIGHT,
        label,
        sender,
        msg
    );
    return but;
}

/// Builds a select directory dialogue button
pub fn build_sel_dir_button(label: &str, sender: Sender<Message>, msg: Message) -> Button {
    return _build_button_adv(
        BUT_SEL_DIR_WIDTH,
        BUT_SEL_DIR_HEIGHT,
        label,
        sender,
        msg
    );
}


/// Callback for handling buttons events.
/// Allows to handle hover events
/// This is a CheckButton version
fn _handle_check_button(b: &mut CheckButton, ev: Event) -> bool {
    return __handle_button_widget(b, ev);
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
/// ev handler, and draw func are pre-defined
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

    return but;
}

// pub fn build_check_button(label: &str, sender: Sender<Message>, msg: Message) -> CheckButton {
//     let but = _build_check_button(
//         BUT_WIDTH,
//         BUT_HEIGHT,
//         label,
//         sender,
//         msg
//     );

//     return but;
// }


/// Builds a frame at the given position
fn _build_frame(xpos: i32, ypos: i32) -> Frame {
    let frame = Frame::default()
        .with_size(INNER_WIN_WIDTH-xpos, INNER_WIN_HEIGHT-ypos)
        .with_pos(xpos, ypos);
        // .with_align(Align::Top | Align::Inside)
        // .with_label(label);

    return frame;
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

    return frame;
}


fn _build_welcome_win_pack() -> Pack {
    const TOTAL_ITEMS: i32 = 2;
    const PAD_X: i32 = 50;
    const PAD_Y: i32 = INNER_WIN_HEIGHT-BUT_HEIGHT-BUT_PACK_YPADDING;

    let mut pack = Pack::default()
        .with_size(INNER_WIN_WIDTH-PAD_X*2, BUT_HEIGHT)
        .with_pos(PAD_X, PAD_Y)
        .with_align(Align::Center)
        .with_type(PackType::Horizontal);
    pack.set_spacing(INNER_WIN_WIDTH - BUT_WIDTH*TOTAL_ITEMS - PAD_X*2);

    pack.end();

    return pack;
}

/// Builds the welcome windows
pub fn build_welcome_win(sender: Sender<Message>) -> DoubleWindow {
    let mut welcome_win = build_inner_win();
    welcome_win.show();
    welcome_win.begin();

    _build_top_frame(WELCOME_FRAME_LABEL);

    let welcome_but_pack = _build_welcome_win_pack();
    welcome_but_pack.begin();

    build_button(BUT_ABORT_LABEL, sender, Message::Abort);
    build_button(BUT_CONTINUE_LABEL, sender, Message::NextPage);

    welcome_but_pack.end();

    welcome_win.end();

    return welcome_win;
}


fn _build_ternary_inner_pack() -> Pack {
    const TOTAL_ITEMS: i32 = 2;

    let mut inner_pack = Pack::default()
        .with_size(BUT_WIDTH*TOTAL_ITEMS + 5, BUT_HEIGHT)
        .with_align(Align::Center)
        .with_type(PackType::Horizontal);
    inner_pack.set_spacing(5);

    inner_pack.end();

    return inner_pack;
}

fn _build_ternary_outer_pack(spacing: i32) -> Pack {
    const PAD_X: i32 = 50;
    const PAD_Y: i32 = INNER_WIN_HEIGHT-BUT_HEIGHT-BUT_PACK_YPADDING;

    let mut pack = Pack::default()
        .with_size(INNER_WIN_WIDTH-PAD_X*2, BUT_HEIGHT)
        .with_pos(PAD_X, PAD_Y)
        .with_align(Align::Center)
        .with_type(PackType::Horizontal);
    pack.set_spacing(INNER_WIN_WIDTH - BUT_WIDTH - spacing - PAD_X*2);

    pack.end();

    return pack;
}

/// Builds a pack of 3 buttons
fn _build_ternary_but_pack(
    sender: Sender<Message>,
    but0_data: (&str, Message),
    but1_data: (&str, Message),
    but2_data: (&str, Message)
) {
    let inner_pack = _build_ternary_inner_pack();

    inner_pack.begin();
    build_button(but1_data.0, sender, but1_data.1);
    build_button(but2_data.0, sender, but2_data.1);
    inner_pack.end();

    let mut outer_pack = _build_ternary_outer_pack(inner_pack.w());

    outer_pack.begin();
    build_button(but0_data.0, sender, but0_data.1);
    outer_pack.add(&inner_pack);
    outer_pack.end();
}

/// Builds a pack of 3 buttons
/// Example: <Abort>      <Back> <Continue>
fn _build_abort_back_contn_pack(sender: Sender<Message>) {
    _build_ternary_but_pack(
        sender,
        (BUT_ABORT_LABEL, Message::Abort),
        (BUT_BACK_LABEL, Message::PrevPage),
        (BUT_CONTINUE_LABEL, Message::NextPage)
    );
}

/// Builds a pack of 3 buttons
/// Example: <Abort>      <Back> <Install>
fn _build_abort_back_inst_pack(sender: Sender<Message>) {
    _build_ternary_but_pack(
        sender,
        (BUT_ABORT_LABEL, Message::Abort),
        (BUT_BACK_LABEL, Message::PrevPage),
        (BUT_INSTALL_LABEL, Message::Install)
    );
}


/// Builds the license window
pub fn build_license_win(sender: Sender<Message>) -> DoubleWindow {
    let license_win = build_inner_win();
    license_win.begin();


    _build_top_frame(LICENSE_FRAME_LABEL);

    let mut buf = TextBuffer::default();
    buf.set_text(APP_LICENSE);

    let mut txt = TextDisplay::default()
        .with_size(INNER_WIN_WIDTH, 310)
        .with_pos(0, 100);
    txt.set_buffer(buf);

    _build_abort_back_contn_pack(sender);


    license_win.end();

    return license_win;
}


/// Builds the select directory window
pub fn build_select_dir_win(sender: Sender<Message>, txt_buf: TextBuffer) -> DoubleWindow {
    let select_dir_win = build_inner_win();
    select_dir_win.begin();


    _build_top_frame(SELECT_DIR_FRAME_LABEL);

    let mut pack = Pack::default()
        .with_size(INNER_WIN_WIDTH, SEL_DIR_TXT_HEIGHT)
        .with_align(Align::Center)
        .with_type(PackType::Horizontal);
    pack = pack.center_of_parent();
    pack.begin();

    let mut txt = TextDisplay::default()
        .with_size(INNER_WIN_WIDTH-BUT_SEL_DIR_WIDTH, SEL_DIR_TXT_HEIGHT);
    txt.set_text_size(SEL_DIR_TXT_SIZE);
    txt.wrap_mode(WrapMode::None, 0);
    txt.set_buffer(txt_buf);

    build_sel_dir_button(BUT_SELECT_DIR_LABEL, sender, Message::SelectDir);

    pack.end();

    _build_abort_back_contn_pack(sender);


    select_dir_win.end();

    return select_dir_win;
}


/// Builds the options window with various settings for installer
pub fn build_options_win(sender: Sender<Message>, is_dlx_version: bool) -> DoubleWindow {
    let options_win = build_inner_win();
    options_win.begin();


    _build_top_frame(OPTIONS_FRAME_LABEL);


    const BUT_USE_DLX_VERSION_WIDTH: i32 = BUT_WIDTH+225;
    const TOTAL_BUTS: i32 = 1;
    const XPOS: i32 = INNER_WIN_WIDTH/2 - BUT_USE_DLX_VERSION_WIDTH/2;
    const YPOS: i32 = INNER_WIN_HEIGHT/2 - TOTAL_BUTS*BUT_HEIGHT/2;

    let mut but_inst_dlx = _build_check_button(
        BUT_USE_DLX_VERSION_WIDTH,
        BUT_HEIGHT,
        BUT_USE_DLX_VERSION_LABEL,
        sender,
        Message::DlxVersionCheck
    );
    but_inst_dlx.set_checked(is_dlx_version);
    but_inst_dlx.set_pos(XPOS, YPOS);


    _build_abort_back_inst_pack(sender);


    options_win.end();

    return options_win;
}


/// Builds a progress bar
pub fn build_progress_bar() -> Progress {
    let mut bar = Progress::default()
        .with_size(PB_WIDTH, PB_HEIGHT)
        .with_pos(0, WIN_HEIGHT/2-PB_HEIGHT/2);
    bar.set_minimum(0.0);
    bar.set_maximum(1.0);
    bar.set_label_font(BUT_FONT);
    bar.set_color(C_WHITE);
    bar.set_selection_color(C_BRIGHT_GREEN);

    return bar;
}

/// Builds the downloading/installing window
pub fn build_propgress_win(sender: Sender<Message>, bar: &Progress) -> DoubleWindow {
    let mut progress_win = build_inner_win();
    progress_win.begin();


    _build_top_frame(PROGRESS_FRAME_LABEL);

    const PAD_X: i32 = 50;
    const PAD_Y: i32 = INNER_WIN_HEIGHT-BUT_HEIGHT-BUT_PACK_YPADDING;
    let mut abrt_but = build_button(BUT_ABORT_LABEL, sender, Message::Abort);
    abrt_but.set_pos(PAD_X, PAD_Y);

    progress_win.add(bar);


    progress_win.end();

    return progress_win;
}


/// Formats a message to
/// text:
///     more text:
///         even more text
fn __format_alert_msg(msg: &str) -> String {
    let mut rv = String::new();
    let mut n: usize = 1;

    for s in msg.split(": ") {
        rv.push_str(s);
        rv.push_str("\n");
        rv.push_str(&"    ".repeat(n));
        n += 1;
    }
    return rv;
}

fn _build_msg_box_ok_but(msg_box_win: &DoubleWindow) -> Button {
    let mut but = _build_button_base(
        BUT_WIDTH,
        BUT_HEIGHT,
        BUT_OK_LABEL,
        _handle_button,
        _draw_button
    );

    but.set_pos(
        ALERT_WIN_WIDTH/2 - BUT_WIDTH/2,
        ALERT_WIN_HEIGHT - BUT_HEIGHT - BUT_ALERT_WIN_PADDING
    );
    but.set_callback({
        let mut win = msg_box_win.clone();
        move |_| win.hide()
    });

    return but;
}

/// Builds an alert window to show a warning to the user
pub fn build_alert_win(msg: &str) -> DoubleWindow {
    let (sw, sh) = screen_size();

    let win_x = sw as i32/2 - ALERT_WIN_WIDTH/2;
    let win_y = sh as i32/2 - ALERT_WIN_HEIGHT/2;

    let mut alert_win = Window::default()
        .with_size(ALERT_WIN_WIDTH, ALERT_WIN_HEIGHT)
        .with_pos(win_x, win_y)
        .with_label(ALERT_WIN_TITLE);
    alert_win.set_color(C_DDLC_WHITE_IDLE);


    let mut buf = TextBuffer::default();
    buf.set_text(
        &__format_alert_msg(msg)
    );

    let mut txt = TextDisplay::default()
        .with_size(
            ALERT_WIN_WIDTH,
            ALERT_WIN_HEIGHT - BUT_HEIGHT - 2*BUT_ALERT_WIN_PADDING
        )
        .with_pos(0, 0);
    txt.set_buffer(buf);


    _build_msg_box_ok_but(&alert_win);


    alert_win.end();
    alert_win.hide();
    alert_win.make_modal(true);

    return alert_win;
}

/// Builds a message box window to show some info to the user
pub fn build_msg_win(msg: &str) -> DoubleWindow {
    let (sw, sh) = screen_size();

    let win_x = sw as i32/2 - ALERT_WIN_WIDTH/2;
    let win_y = sh as i32/2 - ALERT_WIN_HEIGHT/2;

    let mut alert_win = Window::default()
        .with_size(ALERT_WIN_WIDTH, ALERT_WIN_HEIGHT)
        .with_pos(win_x, win_y)
        .with_label(MSG_WIN_TITLE);
    alert_win.set_color(C_DDLC_WHITE_IDLE);


    let mut frame = Frame::default()
        .with_size(ALERT_WIN_WIDTH, ALERT_WIN_HEIGHT - BUT_HEIGHT - 2*BUT_ALERT_WIN_PADDING)
        .with_pos(0, 0)
        .with_align(Align::Center | Align::Inside)
        .with_label(msg);
    frame.set_label_color(C_DDLC_PINK_DARK);
    frame.set_label_size(18);


    _build_msg_box_ok_but(&alert_win);


    alert_win.end();
    alert_win.hide();
    alert_win.make_modal(true);

    return alert_win;
}


fn _build_exit_button(sender: Sender<Message>) -> Button {
    let mut but = build_button(BUT_EXIT_LABEL, sender, Message::Close);
    but.set_pos(INNER_WIN_WIDTH-BUT_WIDTH-50, INNER_WIN_HEIGHT-BUT_HEIGHT-BUT_PACK_YPADDING);

    return but;
}

/// Builds the abort windows
pub fn build_abort_win(sender: Sender<Message>) -> DoubleWindow {
    let abort_win = build_inner_win();
    abort_win.begin();

    _build_top_frame(ABORT_FRAME_LABEL);

    _build_exit_button(sender);

    abort_win.end();

    return abort_win;
}

/// Builds the done windows
pub fn build_done_win(sender: Sender<Message>) -> DoubleWindow {
    let done_win = build_inner_win();
    done_win.begin();

    _build_top_frame(DONE_FRAME_LABEL);

    _build_exit_button(sender);

    done_win.end();

    return done_win;
}
