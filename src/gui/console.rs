use egui_d3d9::EguiDx9;
use std::sync::Once;
use windows::Win32::{
	Foundation::{HWND, LPARAM, LRESULT, WPARAM},
	Graphics::Direct3D9::IDirect3DDevice9,
	UI::WindowsAndMessaging::{CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC},
};

static mut APP: Option<EguiDx9<i32>> = None;

/// Original WNDPROC, so we can do CallWindowProcW(...) with it
static mut FN_ORG_WNDPROC: WNDPROC = None;

fn hello_ui(ctx: &egui::Context, _frame: &mut i32) {
	egui::containers::Window::new("Main").show(ctx, |ui| {
		ctx.settings_ui(ui);
		//ui.label("Hello world!");
		ui.label(egui::RichText::new("Hello world").color(egui::Color32::WHITE));
	});
}

/// Called right before the original IDirect3DDevice9::Present(...) is called.
pub fn draw(dev: &IDirect3DDevice9, hwnd: HWND) {
	static INIT: Once = Once::new();
	INIT.call_once(move || {
		log::trace!("Initializing EguiDx9<_> APP");
		unsafe {
			APP = Some(EguiDx9::init(dev, hwnd, hello_ui, 0, true));
			FN_ORG_WNDPROC = std::mem::transmute(SetWindowLongPtrA(
				hwnd,
				GWLP_WNDPROC,
				hk_wnd_proc as usize as _,
			));
		};
	});
	let app = unsafe { APP.as_mut().unwrap() };
	app.present(dev); //FIXME: Crashes on resize?
}



/// Used reallocate the APP textures that got destroyed during a IDirect3DDevice9::Reset(...)
pub fn reset() {
	if let Some(app) = unsafe { APP.as_mut() } {
		log::trace!("Calling app.reset()...");
		//app.reset(dev);
		app.reset();
		log::trace!("app.reset() returned!!");
	}
}


/// Used to pass GWLP_WNDPROC msg to the EguiDx9 APP (so it can handle clicking / dragging / resizing)
unsafe extern "stdcall" fn hk_wnd_proc(
	hwnd: HWND,
	msg: u32,
	wparam: WPARAM,
	lparam: LPARAM,
) -> LRESULT {
	//This is okay because APP gets set before hooking this WNDPROC
	APP.as_mut().unwrap().wnd_proc(msg, wparam, lparam);
	CallWindowProcW(FN_ORG_WNDPROC, hwnd, msg, wparam, lparam)
}
