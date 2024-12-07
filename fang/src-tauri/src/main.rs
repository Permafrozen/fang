// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Get help syntax:
//      cargo run
// Start just the image editor:
//      cargo run -- --runas-imgeditor
// Start as daemon:
//      cargo run -- --runas-daemon

mod target_platform;
mod grab_image;

use std::env;
use rdev::{listen, Event, EventType, Key};

const RUN_AS_DAEMON_ARG: &str = "--runas-daemon";
const IMAGE_EDITOR_ARG: &str = "--runas-imgeditor";

fn main() {
    // Fancy printout for version
    let version = env!("CARGO_PKG_VERSION");
    println!("Starting FANG {version}");


    // get target we're currently running on
    let target: target_platform::TargetPlatform = target_platform::get_environment();
    println!("Running on platform \"{:?}\"", target);

    
    // CLI arguments
    let args: Vec<String> = env::args().collect();


    // start different modes depending on how fang was started through CLI
    if args.contains(&String::from(RUN_AS_DAEMON_ARG)) {
        
        // Check if tool for capturing screen is available before we start the loop
        if let Err(error_message) = grab_image::check_availability(target) {
            panic!("Error while checking image grabbing tool availability: {}", error_message);
        }

        start_daemon_loop();
    
    }
    else if args.contains(&String::from(IMAGE_EDITOR_ARG)) {
        
        assert!(args.len() == 2, "Insufficient parameters with IMAGE_EDITOR_ARG: path to image as second argument required!");

        // TODO: check if args[1] is a path and if it exists

        open_image_editor(&args[1])

    } else {
        
        println!("Insufficient parameters! Syntax: fang <--runas-daemon|--runas-imgeditor [path]>");
    
    }
}

fn start_daemon_loop() {
    println!("Starting daemon loop");

    // Set up a callback for input
    let callback = |event: Event| {
        match event.event_type {
            EventType::KeyPress(Key::PrintScreen) => {
                println!("Print screen pressed");
                capture_image();
            }
            _ => {}
        }
    };

    // Listen for input indefinite
    let _ = listen(callback);
}

fn capture_image() {
    println!("capturing image...");

    // TODO: let target_platform decide the way it captures the image
}

fn open_image_editor(path: &str) {
    println!("Opening image editor with {path} as target image");

    // TODO: pass path argument to fang GUI image editor and let it handle the rest

    fang_lib::run()
}