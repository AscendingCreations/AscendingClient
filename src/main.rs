#![feature(error_generic_member_access)]
#![allow(
    dead_code,
    clippy::collapsible_match,
    unused_imports,
    clippy::too_many_arguments
)]
use backtrace::Backtrace;
use camera::{
    Projection,
    controls::{Controls, FlatControls, FlatSettings},
};
use cosmic_text::{Attrs, Metrics};
use graphics::*;

use input::{Bindings, FrameTime, InputHandler, Key};
use log::{LevelFilter, Metadata, Record, error, info, warn};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;
use std::{collections::HashMap, env, num::NonZeroUsize};
use std::{
    fs::{self, File},
    io::{Read, Write, prelude::*},
    iter, panic,
    sync::Arc,
    time::{Duration, Instant},
};
use wgpu::{Backends, Dx12Compiler, InstanceDescriptor, InstanceFlags};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::NamedKey,
    platform::windows::WindowAttributesExtWindows,
    window::{WindowAttributes, WindowButtons},
};

mod container;
mod content;
mod data_types;
mod database;
mod runner;
mod systems;
mod widget;

pub use container::*;
use content::*;
pub use data_types::*;
use database::*;
use systems::*;
pub use widget::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Action {
    Quit,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Axis {
    Forward,
    Sideward,
    Yaw,
    Pitch,
}

enum MouseEvent {
    None,
    Click,
    Release,
}

// creates a static global logger type for setting the logger
static MY_LOGGER: MyLogger = MyLogger(log::Level::Trace);
pub const APP_MAJOR: u16 = 0;
pub const APP_MINOR: u16 = 2;
pub const APP_REV: u16 = 0;
pub const SERVER_ID: &str = "127.0.0.1";
pub const SERVER_PORT: u16 = 7010;
pub const TLS_SERVER_PORT: u16 = 7011;

struct MyLogger(pub log::Level);

impl log::Log for MyLogger {
    // checks if it can log these types of events.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.0
    }

    // This logs to a panic file. This is so we can see
    // Errors and such if a program crashes in full render mode.
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{} - {}\n", record.level(), record.args());
            println!("{}", &msg);

            let mut file =
                match File::options().append(true).create(true).open("log.txt")
                {
                    Ok(v) => v,
                    Err(_) => return,
                };

            let _ = file.write(msg.as_bytes());
        }
    }
    fn flush(&self) {}
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load config
    let config = Config::read_config("settings.toml");

    // Create logger to output to a File
    log::set_logger(&MY_LOGGER).unwrap();
    // Set the Max level we accept logging to the file for.
    log::set_max_level(config.level_filter.parse_enum());

    info!("starting up");

    //Comment this out if you do not want a backtrace on error to show.
    if config.enable_backtrace {
        // we will allow this since this is a change coming soon to rust.
        // This can be removed later when it is marked as unsafe.
        #[allow(unused_unsafe)]
        unsafe {
            env::set_var("RUST_BACKTRACE", "1");
        }
    }

    // This allows us to take control of panic!() so we can send it to a file via the logger.
    panic::set_hook(Box::new(|panic_info| {
        let bt = Backtrace::new();

        error!("PANIC: {panic_info}, BACKTRACE: {bt:?}");
    }));

    // we will allow this since this is a change coming soon to rust.
    // This can be removed later when it is marked as unsafe.
    #[allow(unused_unsafe)]
    unsafe {
        env::set_var("WGPU_VALIDATION", "0");
        env::set_var("WGPU_DEBUG", "0");
    }
    // Starts an event gathering type for the window.
    let event_loop = EventLoop::new()?;

    let mut runner = runner::Runner::Loading;
    Ok(event_loop.run_app(&mut runner)?)
}
