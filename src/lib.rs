#![cfg(windows)]

pub mod api;
pub mod dll;
// pub mod gui; //TODO: impl display
pub mod loader;

use std::fs::File;
use std::path::Path;

use simplelog::Config;
use windows::Win32::Foundation::HMODULE;

pub fn init(module: HMODULE) {
	// use windows::Win32::System::Console::AllocConsole;
	// unsafe { AllocConsole().expect("No console?"); };

	let cfg = simplelog::ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();
	let log_file = File::create(Path::new(".").join("amax").join("log").join("d3d9.log"))
		.expect("Couldn't create log file for d3d9.dll");
	simplelog::CombinedLogger::init(vec![
		simplelog::TermLogger::new(
			log::LevelFilter::Trace,
			cfg,
			simplelog::TerminalMode::Mixed,
			simplelog::ColorChoice::Auto,
		),
		simplelog::WriteLogger::new(log::LevelFilter::Trace, Config::default(), log_file),
	])
	.unwrap();

	log_panics::init();

	log::debug!("Hi! {module:?}");

	let r = unsafe { minhook_sys::MH_Initialize() };
	if r != minhook_sys::MH_OK {
		log::error!("Unable to minhook_sys::MH_Initialize() (returned {r})");
	}

	crate::loader::dll_loader::load_dlls()
}

pub fn free(module: HMODULE) {
	log::debug!("Bye! {module:?}");

	crate::api::blur_api::free_plugins();

	let r = unsafe { minhook_sys::MH_Uninitialize() };
	if r != minhook_sys::MH_OK {
		log::error!("Unable to minhook_sys::MH_Uninitialize() (returned {r})");
	}

	// use windows::Win32::System::Console::FreeConsole;
	// unsafe { FreeConsole().expect("Failed to FreeConsole()"); };
}
