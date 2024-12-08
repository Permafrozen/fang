use crate::target_platform::TargetPlatform;
use which::which;
use std::{env}; // temp_dir
use std::path::{Path};

pub fn is_executable_in_path(executable: &str) -> bool {
    which(executable).is_ok()
}

// TODO: doesn't work with nircmd?
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

pub fn get_screentofile_command(platform: TargetPlatform, filepath: &str) -> Result<String, String> {
    let required_executable = match platform {
        TargetPlatform::Windows => format!("{}{}", "nircmd savescreenshot ", filepath),
        TargetPlatform::LinuxWayland => format!("{}{}", "grim ", filepath),
        TargetPlatform::LinuxWaylandGnomeMutter => format!("{}{}", "gnome-screenshot --file=", filepath),
        _ => return Err("Unsupported platform".to_string()),
    };

    Ok(required_executable)
}

pub fn get_temp_screenshot_file_path() -> String {
    let dir = env::temp_dir();
    Path::new(dir.as_os_str())
        .join("fang-tmp-screenshot.png")
        .to_string_lossy()
        .into_owned()
}