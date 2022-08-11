// Include the icon
#[cfg(feature="compile_icon")]
pub static APP_ICON_DATA: &'static [u8] = include_bytes!("static/icon.png");
#[cfg(not(feature="compile_icon"))]
pub static APP_ICON_DATA: &'static [u8] = b"";

// Include license
#[cfg(feature="compile_license")]
pub static APP_LICENSE: &'static str = include_str!("static/license.md");
#[cfg(not(feature="compile_license"))]
pub static APP_LICENSE: &'static str = "";

// Images
pub static VERTICAL_BAR_DATA: &'static [u8] = include_bytes!("static/vertical_bar.png");
pub static VERTICAL_THUMB_DATA: &'static [u8] = include_bytes!("static/vertical_thumb.png");

// Sounds
// Credits: Ludum Dare 28 - Track 1 by @ben_burnes http://abstractionmusic.bandcamp.com/
pub static INSTALLER_THEME_DATA: &'static [u8] = include_bytes!("static/ludum_dare_28_track_1.ogg");
