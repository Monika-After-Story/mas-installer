use fltk::{
    app::{
        App,
        Sender
    },
    button::Button,
    draw,
    enums::{
        Event,
        Font
    },
    prelude::{
        WidgetExt,
        WindowExt,
        GroupExt,
        WidgetBase
    },
    window::{
        Window,
        DoubleWindow
    }
};

use crate::{
    app_styles::*,
    Message
};


/// Builds a default app
pub fn build_app() -> App {
    return App::default()
}


/// Builds a main window
pub fn build_main_win() -> DoubleWindow {
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

    return inner_win
}


/// Callback for handling buttons events.
/// Allows to handle hover events
fn _handle_button(b: &mut Button, ev: Event) -> bool {
    return match ev {
        Event::Enter => {
            b.visible_focus(true);
            b.redraw();
            true
        },
        Event::Leave => {
            b.visible_focus(false);
            b.redraw();
            true
        },
        // For all unhandled events we must return false
        _ => false
    }
}

/// Callback for handling how the button is being drawn.
/// Allows to style it
fn _draw_button(b: &mut Button) {
    let (b_x, b_y, b_w, b_h) = (b.x(), b.y(), b.w(), b.h());

    let (frame_color, bg_color, text_color) = match b.has_visible_focus() {
        true => (C_DDLC_PINK_ACT, C_DDLC_WHITE_ACT, C_PEACH),
        false => (C_DDLC_PINK_IDLE, C_DDLC_WHITE_IDLE, C_BLACK)
    };

    draw::draw_rect_fill(b_x, b_y, b_w, b_h, frame_color);
    draw::draw_rect_fill(b_x+BUT_PADDING, b_y+BUT_PADDING, b_w-BUT_PADDING*2, b_h-BUT_PADDING*2, bg_color);
    draw::set_draw_color(text_color);// for the text
    draw::set_font(Font::HelveticaBold, 18);
    draw::draw_text2(&b.label(), b_x, b_y, b_w, b_h, b.align());
    b.redraw();
}

/// Builds a button with the given label, sender, and msg
/// The button won't be automatically position
pub fn build_button(label: &str, sender: Sender<Message>, msg: Message) -> Button {
    let mut but = Button::default()
        .with_size(BUT_WIDTH, BUT_HEIGHT)
        .with_label(label);

    but.visible_focus(false);
    but.emit(sender, msg);
    but.handle(_handle_button);
    but.draw(_draw_button);
    return but
}
