#![cfg(windows)]

pub mod api;
pub mod dll;
pub mod gui;
pub mod loader;

use std::fs::File;
use std::path::Path;

use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::Console::FreeConsole;

use simplelog::*;

pub fn init(module: HMODULE) {
	unsafe {
		AllocConsole();
	};
	let cfg = ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();
	let log_file = File::create(Path::new(".").join("amax").join("log").join("d3d9.log"))
		.expect("Couldn't create log file for d3d9.dll");
	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			cfg,
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(LevelFilter::Trace, Config::default(), log_file),
	])
	.unwrap();
	log_panics::init();
	log::info!("Hi! {module:?}");
	let r = unsafe { minhook_sys::MH_Initialize() };
	if r != minhook_sys::MH_OK {
		log::error!("Unable to minhook_sys::MH_Initialize() (returned {r})");
	}
}

pub fn free(module: HMODULE) {
	log::info!("Bye! {module:?}");
	crate::api::blur_api::free_plugins();
	let r = unsafe { minhook_sys::MH_Uninitialize() };
	if r != minhook_sys::MH_OK {
		log::error!("Unable to minhook_sys::MH_Uninitialize() (returned {r})");
	}
	unsafe {
		FreeConsole();
	};
}
