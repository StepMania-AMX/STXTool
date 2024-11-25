/// Taken from: https://github.com/frewsxcv/rust-dark-light/blob/main/src/platforms/windows/detect.rs

use winreg::RegKey;

const SUBKEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
const VALUE: &str = "AppsUseLightTheme";

pub fn is_dark_mode() -> bool {
    RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
        .open_subkey(SUBKEY)
        .and_then(|subkey| subkey.get_value::<u32, _>(VALUE))
        .map(|dword| dword == 0)
        .unwrap_or(false)
}
