use fltk::{
    enums::{
        Color
    }
};


// App title
pub const WIN_TITLE: &str = "Monika After Story Installer";

// Window size
pub const WIN_WIDTH: i32 = 600;
pub const WIN_HEIGHT: i32 = 500;

pub const WIN_PADDING: i32 = 4;

// Buttons size
pub const BUT_WIDTH: i32 = 150;
pub const BUT_HEIGHT: i32 = 45;

pub const BUT_PADDING: i32 = 3;

// Color constants
// pub const C_WHITE: Color = Color::White;
pub const C_BLACK: Color = Color::Black;
pub const C_PEACH: Color = Color::from_hex(0xffaa99);
pub const C_DDLC_WHITE_IDLE: Color = Color::from_hex(0xffe6f4);
pub const C_DDLC_PINK_IDLE: Color = Color::from_hex(0xffbde1);
pub const C_DDLC_PINK_DARK: Color = Color::from_hex(0xbb5599);

pub const C_DDLC_WHITE_ACT: Color = Color::from_hex(0xffffff);
pub const C_DDLC_PINK_ACT: Color = C_DDLC_PINK_IDLE;

pub static WELCOME_FRAME_LABEL: &'static str = concat!(
    "Welcome to MAS installer.\n\n",
    "This program will install the Monika After Story\n",
    "mod on your computer."
);
