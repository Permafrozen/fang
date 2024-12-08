use std::env;

// derive debug in order to be able to print out using {:?} inside println macro
#[derive(Debug)]
pub enum TargetPlatform {
    Unknown,
    LinuxNonWayland,
    Windows,
    LinuxWayland,
    LinuxWaylandGnomeMutter
}

fn is_wayland() -> bool {
    env::var("WAYLAND_DISPLAY").is_ok()
}

pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

fn is_gnome_mutter() -> bool {
    let session = env::var("XDG_SESSION_DESKTOP").unwrap_or_default();
    session.to_lowercase().contains("gnome")
}

fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

pub fn get_environment() -> TargetPlatform {
    if is_linux() {
        if is_wayland() {
            if is_gnome_mutter() {
                crate::target_platform::TargetPlatform::LinuxWaylandGnomeMutter
            } else {
                crate::target_platform::TargetPlatform::LinuxWayland
            }
        } else {
            crate::target_platform::TargetPlatform::LinuxNonWayland
        }
    } else if is_windows() {
        crate::target_platform::TargetPlatform::Windows
    } else {
        crate::target_platform::TargetPlatform::Unknown
    }
}