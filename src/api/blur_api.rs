use blur_plugins_core::{BlurAPI, BlurEvent, BlurNotification, BlurPlugin, FnPluginInit};
use windows::{
	core::s,
	Win32::{Foundation::HMODULE, System::LibraryLoader::GetProcAddress},
};

use super::game::get_saved_profile_username;

static mut BLUR_API: MyBlurAPI = MyBlurAPI {
	fps: 0.0,
	plugins: vec![],
};

struct MyBlurAPI {
	fps: f64,
	plugins: Vec<Box<dyn BlurPlugin>>,
}

unsafe impl Send for MyBlurAPI {}
unsafe impl Sync for MyBlurAPI {}

impl MyBlurAPI {
	pub fn register_plugin_from_dll_handle(&mut self, handle: HMODULE) -> bool {
		let fn_plugin_init = unsafe { GetProcAddress(handle, s!("plugin_init")) };
		let Some(fn_plugin_init) = fn_plugin_init else {
			return false;
		};
		// SAFETY: If there is a DLL in amax/dlls/ that exports a plugin_init() function, I simply hope it looks like [`blur_plugins_core::FnPluginInit`] ...
		// More safe would be to check some sort of <BlurPluginVersion> struct to see if this [`BlurAPI`] is compatible with this [`BlurPlugin`]
		let plugin_init: FnPluginInit = unsafe { std::mem::transmute(fn_plugin_init) };
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

	fn notify(&self, notif: BlurNotification) {
		let event = match notif {
			BlurNotification::Nothing => BlurEvent::NoEvent,
			BlurNotification::LoginStart => BlurEvent::LoginStart {
				username: get_saved_profile_username(),
			},
			BlurNotification::LoginEnd { success } => BlurEvent::LoginEnd {
				username: get_saved_profile_username(),
				success,
			},
			BlurNotification::Screen { name } => BlurEvent::Screen { name },
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
	// SAFETY: hhehehehe... No. Any plugin can read or write to the MyBlurAPI fps value at the same time, even across threads!
	// FIXME: Mutex?
	unsafe { BLUR_API.get_fps() }
}

pub fn register_plugin_from_dll_handle(handle: HMODULE) -> bool {
	// SAFETY: <?>: Plugins are guaranteed to load sequentially, after d3d9 stuff is init, and after BLUR_API is init.
	// However, if the plugin does stuff it shouldn't in plugin_init(), this is undefined...
	unsafe { BLUR_API.register_plugin_from_dll_handle(handle) }
}
