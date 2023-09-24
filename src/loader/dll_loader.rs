use std::path::Path;

use windows::{core::HSTRING, Win32::System::LibraryLoader::LoadLibraryW};

pub fn load_dlls() {
	let path_dlls = Path::new(".").join("amax").join("dlls");
	let path_display = path_dlls.display();
	log::info!("Loading DLLs from: {path_display}");
	let entries = path_dlls
		.read_dir()
		.unwrap_or_else(|_| panic!("Could't access [{path_display}] <Path::read_dir() failed> - Does the directory exist?"));
	for entry in entries.filter_map(|e| e.ok()).map(|e| e.path()) {
		let Some(ext) = entry.extension() else {
			continue;
		};
		let Some(ext) = ext.to_str() else {
			continue;
		};
		let entry_display = &entry.display();
		match ext {
			"dll" | "asi" => {
				log::info!("Loading: {entry_display}");
				load_dll(&entry);
			}
			_ => {
				log::info!("Ignoring: {entry_display}");
			}
		};
	}
	log::info!("Done loading DLLs");
}

fn load_dll(dll_path: &Path) {
	let load_result = unsafe { LoadLibraryW(&HSTRING::from(dll_path)) };
	let handle = match load_result {
		Err(err) => {
			let dll_path = dll_path.display();
			log::error!("Error while loading: [{dll_path}]. LoadLibrary() returned: {err:?}");
			panic!("I'd rather panic!() now. Problematic DLL: [{dll_path}]. Error: {err:?}");
		}
		Ok(handle) => {
			handle
		}
	};
	crate::api::blur_api::register_plugin_from_dll_handle(handle);
}
