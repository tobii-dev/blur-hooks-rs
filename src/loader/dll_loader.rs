use std::path::Path;

use windows::{core::HSTRING, Win32::System::LibraryLoader::LoadLibraryW};

pub fn load_dlls() {
	let path_dlls = Path::new(".").join("amax").join("dlls");
	let _path_display = path_dlls.display();
	log::info!("Loading DLLs from: {_path_display}");
	let entries = path_dlls
		.read_dir()
		.unwrap_or_else(|_| panic!("read_dir({_path_display}) failed..."));
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
			log::error!("Error while loading: {dll_path} -- LoadLibrary() returns: {err}");
			//FIXME: We must figure out why it fails sometimes:
			// - tried: LoadLibraryW instead of LoadLibraryA
			// - tried: not callin this from DllMain -.-
			// In the meantime, just panic!()
			panic!("I'd rather crash now. Problematic DLL: {dll_path}; err: {err:?}");
		}
		Ok(handle) => {
			//TODO: Store loaded plugins somewhere, if we are gonna use them later?
			handle
		}
	};
	crate::api::blur_api::register_plugin_from_dll_handle(handle);
}
