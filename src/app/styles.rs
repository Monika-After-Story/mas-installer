/// The module with styles for our app

use std::sync::Mutex;

use fltk::{
    enums::{
        Color,
        Font
    },
    image::PngImage
};

use lazy_static::lazy_static;


// App title
pub const WIN_TITLE: &str = "Monika After Story Installer";
// Err window title
pub const ALERT_WIN_TITLE: &str = "Error!";
// Msg window title
pub const MSG_WIN_TITLE: &str = "Attention!";


// Window consts
pub const WIN_WIDTH: i32 = 600;
pub const WIN_HEIGHT: i32 = 500;

pub const WIN_PADDING: i32 = 4;

pub const INNER_WIN_WIDTH: i32 = WIN_WIDTH - 2*WIN_PADDING;
pub const INNER_WIN_HEIGHT: i32 = WIN_HEIGHT - 2*WIN_PADDING;

pub const ALERT_WIN_WIDTH: i32 = 500;
pub const ALERT_WIN_HEIGHT: i32 = 200;

pub const INNER_ALERT_WIN_WIDTH: i32 = ALERT_WIN_WIDTH - 2*WIN_PADDING;
pub const INNER_ALERT_WIN_HEIGHT: i32 = ALERT_WIN_HEIGHT - 2*WIN_PADDING;

pub const MSG_WIN_WIDTH: i32 = ALERT_WIN_WIDTH;
pub const MSG_WIN_HEIGHT: i32 = ALERT_WIN_HEIGHT;

pub const INNER_MSG_WIN_WIDTH: i32 = MSG_WIN_WIDTH - 2*WIN_PADDING;
pub const INNER_MSG_WIN_HEIGHT: i32 = MSG_WIN_HEIGHT - 2*WIN_PADDING;

pub const INNER_WIN_CONTENT_XPADDING: i32 = 20;
pub const INNER_WIN_CONTENT_YPADDING: i32 = INNER_WIN_CONTENT_XPADDING;


// Buttons consts
pub const BUT_WIDTH: i32 = 130;
pub const BUT_HEIGHT: i32 = 35;

pub const BUT_MUTE_WIDTH: i32 = BUT_HEIGHT;
pub const BUT_MUTE_HEIGHT: i32 = BUT_MUTE_WIDTH;

pub const BUT_DLX_VER_CHECK_WIDTH: i32 = BUT_WIDTH + 225;
pub const BUT_DLX_VER_CHECK_HEIGHT: i32 = BUT_HEIGHT;

pub const BUT_INSTALL_SPR_CHECK_WIDTH: i32 = BUT_WIDTH + 380;
pub const BUT_INSTALL_SPR_CHECK_HEIGHT: i32 = BUT_HEIGHT;

// padding of the frame within buttons
pub const BUT_PADDING: i32 = 3;
// Spacing between teh buttons
pub const BUT_SPACING: i32 = 5;

pub const BUT_FONT_SIZE: i32 = 16;
pub const BUT_FONT: Font = Font::HelveticaBold;

pub const BUT_ABORT_LABEL: &str = "Abort";
pub const BUT_BACK_LABEL: &str = "@< Back ";
pub const BUT_CONTINUE_LABEL: &str = " Continue@>";
pub const BUT_SELECT_DIR_LABEL: &str = "Browse @fileopen";
pub const BUT_DLX_VER_CHECK_LABEL: &str = "Deluxe version (pre-installed spritepacks)";
pub const BUT_INSTALL_SPR_CHECK_LABEL: &str = "Download spritepacks (separate download into '/spritepacks')";
pub const BUT_INSTALL_LABEL: &str = "Install";
pub const BUT_OK_LABEL: &str = "Ok";
pub const BUT_EXIT_LABEL: &str = "Exit";

pub const BUT_ALERT_WIN_PADDING: i32 = 10;
pub const BUT_MSG_WIN_PADDING: i32 = BUT_ALERT_WIN_PADDING;
pub const BUT_PACK_YPADDING: i32 = INNER_WIN_CONTENT_YPADDING;


// Frame consts
pub const TOP_FRAME_LABEL_SIZE: i32 = LABEL_SIZE_LARGE;
pub const TOP_FRAME_XPOS: i32 = 0;
pub const TOP_FRAME_YPOS: i32 = INNER_WIN_CONTENT_YPADDING;
pub const TOP_FRAME_WIDTH: i32 = INNER_WIN_WIDTH;
pub const TOP_FRAME_HEIGHT: i32 = 35;

pub const MID_FRAME_XPOS: i32 = TOP_FRAME_XPOS;
pub const MID_FRAME_YPOS: i32 = 2*TOP_FRAME_YPOS + TOP_FRAME_HEIGHT;
pub const MID_FRAME_WIDTH: i32 = TOP_FRAME_WIDTH;
pub const MID_FRAME_HEIGHT: i32 = INNER_WIN_HEIGHT - MID_FRAME_YPOS - BUT_HEIGHT - 2*INNER_WIN_CONTENT_YPADDING;
pub const MID_FRAME_LABEL_SIZE: i32 = TOP_FRAME_LABEL_SIZE;

pub const MSG_FRAME_LABEL_SIZE: i32 = LABEL_SIZE_MED;


// Text display constants
pub const TXT_DISP_XPOS: i32 = INNER_WIN_CONTENT_XPADDING;
pub const TXT_DISP_YPOS: i32 = MID_FRAME_YPOS;
pub const TXT_DISP_WIDTH: i32 = INNER_WIN_WIDTH - 2*INNER_WIN_CONTENT_XPADDING;
pub const TXT_DISP_HEIGHT: i32 = MID_FRAME_HEIGHT;


// Text consts
pub const SEL_DIR_TXT_XPOS: i32 = INNER_WIN_CONTENT_XPADDING;
pub const SEL_DIR_TXT_YPOS: i32 = INNER_WIN_HEIGHT/2 - SEL_DIR_TXT_HEIGHT - BUT_SPACING/2;
pub const SEL_DIR_TXT_WIDTH: i32 = INNER_WIN_WIDTH - 2*INNER_WIN_CONTENT_XPADDING;
pub const SEL_DIR_TXT_HEIGHT: i32 = 28;
pub const SEL_DIR_TXT_SIZE: i32 = 18;

pub const SEL_DIR_DLG_PROMPT: &str = "Select Doki Doki Literature Club directory";

pub const LABEL_SIZE_LARGE: i32 = 28;
pub const LABEL_SIZE_MED: i32 = 20;


// Progress bar consts
pub const PB_WIDTH: i32 = INNER_WIN_WIDTH - 2*INNER_WIN_CONTENT_XPADDING;
pub const PB_HEIGHT: i32 = BUT_HEIGHT;


// Slider consts
pub const SCROLL_AMOUNT: f64 = 3.0;
// The number of characters to ignore by the slider,
// there doesn't appear to be a better way, this value works for us
// so we hard-code it
pub const LICENSE_SLIDER_LINES_IGNORE: i32 = 910;
pub const LICENSE_SLIDER_WIDTH: i32 = INNER_WIN_CONTENT_XPADDING;
pub const LICENSE_SLIDER_HEIGHT: i32 = TXT_DISP_HEIGHT;
pub const LICENSE_SLIDER_SIZE: f32 = 0.2;


// Color constants
pub const C_BLACK: Color = Color::Black;
pub const C_WHITE: Color = Color::White;

pub const C_DDLC_PEACH: Color = Color::from_hex(0xffaa99);
pub const C_DDLC_WHITE_IDLE: Color = Color::from_hex(0xffe6f4);
pub const C_DDLC_PINK_IDLE: Color = Color::from_hex(0xffbde1);
pub const C_DDLC_PINK_DARK: Color = Color::from_hex(0xbb5599);

pub const C_DDLC_WHITE_ACT: Color = Color::from_hex(0xffffff);
pub const C_DDLC_PINK_ACT: Color = C_DDLC_PINK_IDLE;

pub const C_BRIGHT_GREEN: Color = Color::from_hex(0x00ff00);


// Raw text
pub const WELCOME_TOP_FRAME_LABEL: &str = "Welcome to MAS installer";
pub const WELCOME_MID_FRAME_LABEL: &str = concat!(
    "This program will install the latest version\n",
    "of the Monika After Story mod\n",
    "on your computer"
);
pub const LICENSE_FRAME_LABEL: &str = "By continuing you agree with our license";
pub const SELECT_DIR_FRAME_LABEL: &str = "Select Doki Doki Literature Club directory";
pub const OPTIONS_FRAME_LABEL: &str = "Select additional settings";
pub const PROGRESS_FRAME_LABEL: &str = "Installing. Please wait";
pub const ABORT_TOP_FRAME_LABEL: &str = "Aborted";
pub const ABORT_MID_FRAME_LABEL: &str = concat!(
    "Installation has been aborted.\n",
    "Any already extracted files will remain"
);
pub const DONE_TOP_FRAME_LABEL: &str = "Finished";
pub const DONE_MID_FRAME_LABEL: &str = concat!(
    "Monika After Story has been successfully\n",
    "installed on your computer"
);


// Define images
lazy_static! {
    pub static ref VOLUME_BUT_IMG: Mutex<PngImage> = Mutex::new(
        PngImage::from_data(crate::static_data::VOLUME_BUT_DATA).unwrap()
    );
    pub static ref VOLUME_BUT_HOVER_IMG: Mutex<PngImage> = Mutex::new(
        PngImage::from_data(crate::static_data::VOLUME_BUT_HOVER_DATA).unwrap()
    );
}
