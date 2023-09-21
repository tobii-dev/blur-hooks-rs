use blur_plugins_core::{BlurAPI, BlurEvent, BlurPlugin, FnInit};
use windows::{
	core::s,
	Win32::{Foundation::HMODULE, System::LibraryLoader::GetProcAddress},
};

static mut BLUR_API: MyBlurAPI = MyBlurAPI {
	fps: 0.0,
	plugins: vec![],
};

struct MyBlurAPI {
	fps: f64,
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

	fn notify(&self, event: BlurEvent) {
		// FIXME: this is ugly...
		// maybe better just "get profile username" fn for BlurAPI
		let event = if let BlurEvent::LoginStart { .. } = event {
			BlurEvent::LoginStart {
				username: crate::api::game::get_saved_profile_username(),
			}
		} else {
			event
		};
		for plugin in &self.plugins {
			plugin.on_event(&event);
		}
	}
}

pub fn free_plugins() {
	// SAFETY: lol no
	unsafe {
		BLUR_API.free_plugins();
	};
}

pub fn get_fps() -> f64 {
	// SAFETY: hhehehehe nope. Any plugin can read or write to the MyBlurAPI fps value at the same time.
	unsafe { BLUR_API.get_fps() }
}

pub fn register_plugin_from_dll_handle(handle: HMODULE) -> bool {
	// SAFETY: Some: Plugins are guaranteed to load sequentially, after d3d9 stuff is init, after BLUR_API is init.
	// However, if the plugin does stuff it shouldn't in plugin_init(), IDK what happens...
	unsafe { BLUR_API.register_plugin_from_dll_handle(handle) }
}
