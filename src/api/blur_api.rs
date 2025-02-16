use std::{
	ffi::c_void,
	sync::{LazyLock, Mutex},
};

use blur_plugins_core::{BlurAPI, BlurEvent, BlurNotification, BlurPlugin, FnPluginInit};
use windows::{
	core::s,
	Win32::{
		Foundation::HMODULE, Graphics::Direct3D9::IDirect3DDevice9,
		System::LibraryLoader::GetProcAddress,
	},
};

use super::{
	fps::FpsLimiter,
	game::{self},
};

struct MyBlurAPI {
	fps_limiter: FpsLimiter,
	plugins: Vec<Box<dyn BlurPlugin>>,
	d3d9dev: *mut IDirect3DDevice9,
	ptr_base: *mut std::ffi::c_void,
}

unsafe impl Send for MyBlurAPI {}
unsafe impl Sync for MyBlurAPI {}

static G_BLUR_API: LazyLock<Mutex<MyBlurAPI>> = LazyLock::new(|| {
	//TODO: Consider init after d3d9dev initialized
	MyBlurAPI {
		fps_limiter: FpsLimiter::new(),
		plugins: vec![],
		d3d9dev: std::ptr::null_mut(),
		ptr_base: game::get_exe_module_ptr(),
	}
	.into()
});

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
		let name = plugin.name();
		log::info!("Loaded plugin: {name}");
		self.plugins.push(plugin);
	}

	fn free_plugins(&mut self) {
		while let Some(plugin) = self.plugins.pop() {
			let name = plugin.name();
			log::info!("Unloading plugin: {name}");
			plugin.free();
		}
	}

	/// Send event to all plugins
	fn dispatch(&self, event: BlurEvent) {
		log::debug!("Sending event: {event:?} to all plugins.");
		for plugin in &self.plugins {
			plugin.on_event(&event);
		}
	}
}

impl BlurAPI for MyBlurAPI {
	fn set_fps(&mut self, fps: f64) -> bool {
		let fps = if 0.0 < fps { Some(fps) } else { None };
		self.fps_limiter.set_fps(fps);
		fps.is_some()
	}

	fn get_fps(&self) -> f64 {
		self.fps_limiter.get_fps()
	}

	fn register_event(&mut self, _event: &BlurEvent) {
		todo!();
	}

	fn notify(&self, notif: BlurNotification) {
		let event = match notif {
			BlurNotification::Nothing => BlurEvent::NoEvent,
			BlurNotification::LoginStart => BlurEvent::LoginStart {
				username: self.get_saved_profile_username(),
			},
			BlurNotification::LoginEnd { success } => BlurEvent::LoginEnd {
				username: self.get_saved_profile_username(),
				success,
			},
			BlurNotification::Screen { name } => BlurEvent::Screen { name },
			BlurNotification::PluginStuff { id, data } => {
				log::trace!("MyBlurAPI got notify for PluginStuff:{id}");
				BlurEvent::PluginData { id, data }
			}
		};
		self.dispatch(event);
	}

	fn get_d3d9dev(&self) -> *mut std::ffi::c_void {
		(&self).d3d9dev as *mut std::ffi::c_void
	}

	fn get_exe_base_ptr(&self) -> *mut std::ffi::c_void {
		self.ptr_base
	}

	fn get_saved_profile_username(&self) -> String {
		game::read_saved_profile_username(self.ptr_base)
	}
}

pub fn limit_fps() {
	G_BLUR_API.lock().unwrap().fps_limiter.limit_fps();
}

pub fn free_plugins() {
	G_BLUR_API.lock().unwrap().free_plugins();
}

pub fn get_fps() -> f64 {
	G_BLUR_API.lock().unwrap().get_fps()
}

pub fn set_d3d9dev(dev_ptr: *mut IDirect3DDevice9) {
	log::trace!("G_BLUR_API: set_d3d9dev(.={dev_ptr:?})");
	{
		// I am scared of using if-let with mutex
		let mut api = G_BLUR_API.lock().unwrap();
		api.d3d9dev = dev_ptr;
		api.dispatch(BlurEvent::Direct3DInit {
			dev_ptr: dev_ptr as *mut c_void,
		});
	}
}

pub fn register_plugin_from_dll_handle(handle: HMODULE) -> bool {
	G_BLUR_API
		.lock()
		.unwrap()
		.register_plugin_from_dll_handle(handle)
}
