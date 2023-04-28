use std::path::Path;

use windows::{
	core::PCSTR,
	Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA},
};

pub fn load_dlls() {
	let path_dlls = Path::new(".").join("amax").join("dlls");
	let _path_display = path_dlls.display();
	log::info!("Loading dlls from: {_path_display}");
	let entries = path_dlls.read_dir().expect("read_dir() failed...");
	for entry in entries.filter_map(|e| e.ok()).map(|e| e.path()) {
		if let Some(ext) = entry.extension() {
			let ext = ext.to_str().unwrap();
			match ext {
				"dll" | "asi" => {
					load(&entry);
				}
				_ => {
					let l = entry.display();
					log::info!("ignoring: {l}");
				}
			};
		}
	}
}

/// This is called (every frame) from d3d9 Device Present() hook to limit fps
pub static mut FN_GET_FPS: Option<extern "C" fn() -> u32> = None;

fn load(dll_path: &Path) {
	/* FIXME: why does this not throw errors at me sometimes and not other times?
	use libloading::{Library, Symbol};
	let lib = unsafe { Library::new(dll_path.as_os_str()) };
	match lib {
		Ok(lib) => {
			let uwu: Result<Symbol<extern "C" fn(i32) -> i32>, _> = unsafe { lib.get(b"loader_uwu") };
			match uwu {
				Ok(loader_uwu) => {
					let x = loader_uwu(2);
					log::trace!("loader_uwu(2) = {x}");
				},
				Err(err) => {
					log::error!("{err}");
				},
			}
		},
		Err(err) => {
			log::error!(": {err}");
		},
	};
	*/

	//TODO: Find a clean way to communicate between dlls and d3d9 stuff.

	let dll_path = dll_path.as_os_str().to_str().unwrap();
	let load_result = unsafe { LoadLibraryA(PCSTR::from_raw(dll_path.as_ptr())) };
	let handle = match load_result {
		Err(err) => {
			log::error!("Error while loading: {dll_path} (LoadLibraryA() returns {err})");
			return;
		}
		Ok(handle) => {
			//TODO: Store loaded plugins somewhere, if we are gonna use them later?
			handle
		}
	};
	// FIXME: Okay... for now this works, but seems ugly heck
	let fn_get_fps = unsafe { GetProcAddress(handle, windows::s!("get_fps")) };
	if let Some(fn_get_fps) = fn_get_fps {
		let get_fps: extern "C" fn() -> u32 = unsafe { std::mem::transmute(fn_get_fps) };
		unsafe { FN_GET_FPS = Some(get_fps) };
	}
}
