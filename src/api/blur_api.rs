use blur_plugins_core::{BlurAPI, BlurEvent, BlurPlugin, FnInit};
use windows::{
	s,
	Win32::{Foundation::HMODULE, System::LibraryLoader::GetProcAddress},
};

///FIXME: This should never ever ever ever ever EVER be pub
///Better would be to create a function that gets a reference to this or something idk...
pub static mut BLUR_API: MyBlurAPI = MyBlurAPI {
	fps: 0.0,
	plugins: vec![],
};

pub struct MyBlurAPI {
	pub fps: f64,
	plugins: Vec<Box<dyn BlurPlugin>>,
}

impl MyBlurAPI {
	pub fn register_plugin_from_dll_handle(&mut self, handle: HMODULE) -> bool {
		let fn_plugin_init = unsafe { GetProcAddress(handle, s!("plugin_init")) };
		let Some(fn_plugin_init) = fn_plugin_init else {
			return false;
		};
		let plugin_init: FnInit = unsafe { std::mem::transmute(fn_plugin_init) };
		let plugin = plugin_init(self);
		self.register_plugin(plugin);
		true
	}

	/*
	#[deprecated]
	pub fn register_plugin_from_libloading_library(&mut self, lib: libloading::Library) -> bool {
		let uwu: Result<libloading::Symbol<FnInit>, _> = unsafe { lib.get(b"plugin_init") };
		match uwu {
			Ok(loader_uwu) => {
				let plugin = loader_uwu(self);
				self.register_plugin(plugin);
				true
			}
			Err(err) => {
				log::error!("{err}");
				false
			}
		}
	}
	*/

	fn register_plugin(&mut self, plugin: Box<dyn BlurPlugin>) {
		self.plugins.push(plugin);
	}

	fn free_plugins(&mut self) {
		while let Some(plugin) = self.plugins.pop() {
			let name = plugin.name();
			log::info!("Unloading plugin: {name}");
			plugin.free();
		}
	}
}

impl BlurAPI for MyBlurAPI {
	fn set_fps(&mut self, fps: f64) -> bool {
		self.fps = fps;
		true
	}

	fn get_fps(&self) -> f64 {
		self.fps
	}

	fn register_event(&mut self, _event: &BlurEvent) {
		todo!();
	}

	fn notify(&self, event: &BlurEvent) {
		for plugin in &self.plugins {
			plugin.on_event(event);
		}
	}
}

pub fn free_plugins() {
	unsafe {
		BLUR_API.free_plugins();
	};
}
