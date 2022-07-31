use fltk::{
    enums::{
        Color,
        Font
    }
};


// App title
pub const WIN_TITLE: &str = "Monika After Story Installer";


// Window consts
pub const WIN_WIDTH: i32 = 600;
pub const WIN_HEIGHT: i32 = 500;

pub const WIN_PADDING: i32 = 4;


// Buttons consts
pub const BUT_WIDTH: i32 = 130;
pub const BUT_HEIGHT: i32 = 35;

pub const BUT_SEL_DIR_WIDTH: i32 = 160;
pub const BUT_SEL_DIR_HEIGHT: i32 = 42;

pub const BUT_PADDING: i32 = 3;

pub const BUT_FONT_SIZE: i32 = 16;
pub const BUT_FONT: Font = Font::HelveticaBold;

pub const BUT_ABORT_LABEL: &str = "Abort";
pub const BUT_BACK_LABEL: &str = "@< Back ";
pub const BUT_CONTINUE_LABEL: &str = " Continue@>";
pub const BUT_SELECT_DIR_LABEL: &str = " Select directory @fileopen";
pub const BUT_USE_DLX_VERSION_LABEL: &str = "Deluxe version (pre-installed spritepacks)";
pub const BUT_INSTALL_LABEL: &str = " Install";


// Frame consts
pub const TOP_FRAME_LABEL_SIZE: i32 = 28;
pub const TOP_FRAME_XPOS: i32 = 0;
pub const TOP_FRAME_YPOS: i32 = 15;


// Text consts
pub const SEL_DIR_TXT_HEIGHT: i32 = BUT_SEL_DIR_HEIGHT;
pub const SEL_DIR_TXT_SIZE: i32 = 18;

pub const SEL_DIR_DLG_PROMPT: &str = "Select DDLC directory";


// Color constants
// pub const C_WHITE: Color = Color::White;
pub const C_BLACK: Color = Color::Black;
pub const C_DDLC_PEACH: Color = Color::from_hex(0xffaa99);
pub const C_DDLC_WHITE_IDLE: Color = Color::from_hex(0xffe6f4);
pub const C_DDLC_PINK_IDLE: Color = Color::from_hex(0xffbde1);
pub const C_DDLC_PINK_DARK: Color = Color::from_hex(0xbb5599);

pub const C_DDLC_WHITE_ACT: Color = Color::from_hex(0xffffff);
pub const C_DDLC_PINK_ACT: Color = C_DDLC_PINK_IDLE;


pub const WELCOME_FRAME_LABEL: &str = concat!(
    "Welcome to MAS installer\n\n\n\n\n",
    "This program will install the Monika After Story\n",
    "mod on your computer"
);
pub const LICENSE_FRAME_LABEL: &str = "By continuing you agree with our license";
pub const SELECT_DIR_FRAME_LABEL: &str = "Select Doki Doki Literature Club directory";
pub const OPTIONS_FRAME_LABEL: &str = "Select additional settings";
