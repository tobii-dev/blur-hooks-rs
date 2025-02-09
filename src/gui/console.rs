use std::sync::OnceLock;

use egui::mutex::Mutex;
use egui_d3d9::EguiDx9;
use windows::Win32::{
	Foundation::{HWND, LPARAM, LRESULT, WPARAM},
	Graphics::Direct3D9::IDirect3DDevice9,
	UI::WindowsAndMessaging::{CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC},
};

struct MyApp {
	gui: EguiDx9<MyState>,
}

struct MyState {
	v: Vec<String>,
	should_display: bool,
}

// TODO: impl display
impl MyApp {
	unsafe fn new(dev: &IDirect3DDevice9) -> windows::core::Result<Self> {
		let pparameters =
			&mut windows::Win32::Graphics::Direct3D9::D3DDEVICE_CREATION_PARAMETERS::default();
		unsafe { dev.GetCreationParameters(pparameters) }?;
		let hwnd = pparameters.hFocusWindow;
		let my_state = MyState {
			v: vec![],
			should_display: true,
		};
		Self::set_wndproc_hook(hwnd);
		Ok(Self {
			gui: EguiDx9::init(dev, hwnd, hello_ui, my_state, true),
		})
	}

	fn set_wndproc_hook(hwnd: HWND) {
		unsafe {
			FN_ORG_WNDPROC = std::mem::transmute(SetWindowLongPtrA(
				hwnd,
				GWLP_WNDPROC,
				Self::wndproc_hook as usize as _,
			));
		}
	}

	/// Used to pass GWLP_WNDPROC msg to the EguiDx9 APP (so it can handle clicking / dragging / resizing)
	unsafe extern "stdcall" fn wndproc_hook(
		hwnd: HWND,
		msg: u32,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		if let Some(app) = G_APP.get() {
			app.lock().gui.wnd_proc(msg, wparam, lparam);
		}
		CallWindowProcW(FN_ORG_WNDPROC, hwnd, msg, wparam, lparam)
	}
}

/// SAFETY: I'm not sure. It didn't crash when it was a &static mut ref to EguiDx9<>, so it probably won't crash now.
/// I think we are the only thread touching the G_APP anyways, and we do so from behind a Mutex.
/// If IDirect3DDevice9::Present() is called from other threads, and internally EguiDx9 needs references from there, stuff might crash.
/// So far it hasn't... :D
unsafe impl Send for MyApp {}
unsafe impl Sync for MyApp {} // <?>

/// Performance: This needs to lock() & unlock() a Mutex every game frame, I'm not sure if that is worth it.
/// I only do this to shut up the compiler - #[warn(static_mut_refs)]
/// I think it would be ok to just have G_APP stored as a static mut EguiDx9<>.
/// The problem with that static mut global that is initialization and access gets kinda ugly (MaybeUninit<>, assume_init_mut()...)
static G_APP: OnceLock<Mutex<MyApp>> = OnceLock::new();

/// Original WNDPROC, so we can do CallWindowProcW(...) with it
static mut FN_ORG_WNDPROC: WNDPROC = None;

/// I find this so ugly...
fn hello_ui(ctx: &egui::Context, state: &mut MyState) {
	if !state.should_display {
		return;
	}
	egui::containers::Window::new("Main").show(ctx, |ui| {
		for line in state.v.iter().map(|v| v.as_str()) {
			ui.label(line);
		}
		let fps = crate::api::blur_api::get_fps();
		ui.label(fps.to_string());
	});
}

pub fn draw(dev: &IDirect3DDevice9) {
	let app = G_APP.get_or_init(|| {
		let my_app = unsafe { MyApp::new(dev) }.unwrap();
		Mutex::new(my_app)
	});
	app.lock().gui.present(dev);
}

/// Reset APP resources that were destroyed during IDirect3DDevice9::Reset(...)
pub fn reset() {
	if let Some(app) = G_APP.get() {
		app.lock().gui.pre_reset();
	}
}
