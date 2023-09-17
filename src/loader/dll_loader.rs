use std::path::Path;

use windows::{core::PCSTR, Win32::System::LibraryLoader::LoadLibraryA};

pub fn load_dlls() {
	let path_dlls = Path::new(".").join("amax").join("dlls");
	let _path_display = path_dlls.display();
	log::info!("Loading dlls from: {_path_display}");
	let entries = path_dlls
		.read_dir()
		.unwrap_or_else(|_| panic!("read_dir({_path_display}) failed..."));
	for entry in entries.filter_map(|e| e.ok()).map(|e| e.path()) {
		if let Some(ext) = entry.extension() {
			let ext = ext.to_str().unwrap();
			match ext {
				"dll" | "asi" => {
					log::info!("Loading dll: {}", &entry.display());
					load(&entry);
					//libloading_load_dll(&entry); // Y U NO WORK :(
				}
				_ => {
					let l = entry.display();
					log::info!("ignoring: {l}");
				}
			};
		}
	}
}

#[allow(dead_code)]
fn load(dll_path: &Path) {
	//TODO: Find a clean way to communicate between dlls and d3d9 stuff.
	let dll_path = dll_path.as_os_str().to_str().unwrap();
	let load_result = unsafe { LoadLibraryA(PCSTR::from_raw(dll_path.as_ptr())) };
	let handle = match load_result {
		Err(err) => {
			log::error!("Error while loading: {dll_path} -- LoadLibraryA() returns: {err}");
			//FIXME: We must figure out why it fails.
			//In the meantime, just panic!()
			//return;
			panic!("I'd rather crash now. Problematic DLL: {dll_path}; err: {err:?}");
		}
		Ok(handle) => {
			//TODO: Store loaded plugins somewhere, if we are gonna use them later?
			handle
		}
	};
	unsafe { crate::api::blur_api::BLUR_API.register_plugin_from_dll_handle(handle) };
}

#[deprecated]
pub fn libloading_load_dll(dll_path: &Path) {
	let lib = unsafe { libloading::Library::new(dll_path.as_os_str()) };
	let _lib = match lib {
		Ok(lib) => lib,
		Err(err) => {
			log::error!(": {err}");
			return;
		}
	};
	//unsafe { crate::api::blur_api::BLUR_API.register_plugin_from_libloading_library(lib) };
}
