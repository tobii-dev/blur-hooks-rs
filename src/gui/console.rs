use egui::Context;
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

fn hello_ui(ctx: &Context, _frame: &mut i32) {
	log::trace!("I made it to hello_ui(..., _frame={_frame})"); // it reaches this line...
	egui::containers::Window::new("Main").show(ctx, |ui| {
		ctx.settings_ui(ui);
		ui.label("Hello world!");
		//ui.label(egui::RichText::new("Hello world").color(egui::Color32::WHITE));
	});
	log::trace!("I made past egui::containers::Window::new(...)!"); // and this one... But after that it crashes?
}

/// draw() gets called right before the original IDirect3DDevice9::Present() is called.
pub fn draw(dev: &IDirect3DDevice9, hwnd: HWND) {
	log::trace!("I made it to draw({hwnd:?})");
	static INIT: Once = Once::new();
	INIT.call_once(move || {
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
	app.present(dev); //Crashes
	log::trace!("I made it past app.present(dev))"); // doesnt reach this line I think
}

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
