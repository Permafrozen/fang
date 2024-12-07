use crate::target_platform::TargetPlatform;
use which::which;

pub fn is_executable_in_path(executable: &str) -> bool {
    which(executable).is_ok()
}

pub fn check_availability(platform: TargetPlatform) -> Result<(), String> {
    let required_executable = match platform {
        TargetPlatform::Windows => "nircmd",
        TargetPlatform::LinuxWayland => "grim",
        TargetPlatform::LinuxWaylandGnomeMutter => "gnome-screenshot",
        _ => return Err("Unsupported platform".to_string()),
    };

    if is_executable_in_path(required_executable) {
        Ok(())
    } else {
        Err(format!("{} not found in PATH", required_executable))
    }
}