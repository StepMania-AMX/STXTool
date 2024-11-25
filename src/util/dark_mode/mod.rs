use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
use linux::is_dark_mode as probe_os_for_dark_mode;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "macos")]
use macos::is_dark_mode as probe_os_for_dark_mode;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
use windows::is_dark_mode as probe_os_for_dark_mode;

static mut DARK_MODE_CACHE: Option<(bool, Instant)> = None;

pub fn is_dark_mode() -> bool {
    unsafe {
        match DARK_MODE_CACHE {
            Some((cached_value, last_check_time))
                if last_check_time.elapsed() < Duration::from_secs(1) =>
            {
                cached_value
            }
            _ => {
                let result = probe_os_for_dark_mode();
                DARK_MODE_CACHE.replace((result, Instant::now()));
                result
            }
        }
    }
}
