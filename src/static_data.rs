/// The module with static data of our app

// Include the icon
pub static APP_ICON_DATA: &'static [u8] = include_bytes!("static/icon.png");

// Include license
#[cfg(feature="include_license")]
pub static APP_LICENSE: &'static str = include_str!("static/license.md");
#[cfg(not(feature="include_license"))]
pub static APP_LICENSE: &'static str = "You can find the license at https://github.com/Monika-After-Story/MonikaModDev/blob/master/LICENSE.md";

// Images
pub static VERTICAL_BAR_DATA: &'static [u8] = include_bytes!("static/vertical_bar.png");
pub static VERTICAL_THUMB_DATA: &'static [u8] = include_bytes!("static/vertical_thumb.png");
pub static VOLUME_BUT_DATA: &'static [u8] = include_bytes!("static/but_volume.png");
pub static VOLUME_BUT_HOVER_DATA: &'static [u8] = include_bytes!("static/but_volume_hover.png");

// Sounds
// Credits: Ludum Dare 28 - Tracks 1 and 3 by @ben_burnes http://abstractionmusic.bandcamp.com/
pub static INSTALLER_THEME_DATA: &'static [u8] = include_bytes!("static/ludum_dare_28_track_3.ogg");
