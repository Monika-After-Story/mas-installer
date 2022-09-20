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
pub static VOLUME_BUT_CHECK_DATA: &'static [u8] = include_bytes!("static/but_volume_check.png");
pub static VOLUME_BUT_CHECK_HOVER_DATA: &'static [u8] = include_bytes!("static/but_volume_check_hover.png");
pub static VOLUME_BUT_UNCHECK_DATA: &'static [u8] = include_bytes!("static/but_volume_uncheck.png");
pub static VOLUME_BUT_UNCHECK_HOVER_DATA: &'static [u8] = include_bytes!("static/but_volume_uncheck_hover.png");

// Sounds
// Credits: Doki Doki Literature Club - Main Theme (Your Reality) (8-bit Remix)
// by MyNewSoundtrack https://www.youtube.com/user/MyNewSoundtrack
pub static INSTALLER_THEME_DATA: &'static [u8] = include_bytes!("static/installer_theme.ogg");
