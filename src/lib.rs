#![cfg(windows)]

pub mod dll;

use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::Console::AllocConsole;

use log::{error, info};
use simplelog::*;

pub fn init(module: HMODULE) {
    unsafe {
        AllocConsole();
    };
    let cfg = ConfigBuilder::new()
        .set_time_offset_to_local()
        .unwrap()
        .build();
    let _logger = CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
            cfg,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            std::fs::File::create("d3d9.log").unwrap(),
        ),
    ])
    .unwrap();
    log_panics::init();
    info!("Hi! {module:?}");
    let r = unsafe { minhook_sys::MH_Initialize() };
    if r != minhook_sys::MH_OK {
        error!("Unable to minhook_sys::MH_Initialize() (returned {r})");
    }
}
