use simplelog::Config;
use windows::Win32::Foundation::HMODULE;

pub(in crate::dll) fn init(module: HMODULE) {
	unsafe {
		use windows::Win32::System::Console::AllocConsole;
		AllocConsole().expect("No console?");
	};

	let cfg = simplelog::ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();

	let log_file = blur_plugins_core::create_log_file("d3d9.log").unwrap_or_else(|e| {
		panic!("Couldn't create log file: {e:?}");
	});
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

pub(in crate::dll) fn free(module: HMODULE) {
	log::debug!("Bye! {module:?}");

	crate::api::blur_api::free_plugins();

	let r = unsafe { minhook_sys::MH_Uninitialize() };
	if r != minhook_sys::MH_OK {
		log::error!("Unable to minhook_sys::MH_Uninitialize() (returned {r})");
	}

	unsafe {
		use windows::Win32::System::Console::FreeConsole;
		FreeConsole().expect("Failed to FreeConsole()");
	};
}
