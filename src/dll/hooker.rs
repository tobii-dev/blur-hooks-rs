#![allow(non_snake_case, non_camel_case_types)]
use std::ffi::c_void;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::Graphics::Direct3D9::IDirect3DBaseTexture9;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;
use windows::Win32::Graphics::Direct3D9::IDirect3DTexture9;
use windows::Win32::Graphics::Direct3D9::D3DFORMAT;
use windows::Win32::Graphics::Direct3D9::D3DPOOL;
use windows::Win32::Graphics::Direct3D9::D3DPRESENT_PARAMETERS;
use windows::Win32::Graphics::Direct3D9::D3DQUERYTYPE;

use windows::Win32::Graphics::Gdi::RGNDATA;

use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::RECT;

use windows::core::Interface;
use windows::core::HRESULT;

type FnEndScene = unsafe extern "system" fn(this: IDirect3DDevice9) -> HRESULT;

type FnPresent = unsafe extern "system" fn(
	this: IDirect3DDevice9,
	psourcerect: *const RECT,
	pdestrect: *const RECT,
	hdestwindowoverride: HWND,
	pdirtyregion: *const RGNDATA,
) -> HRESULT;

type FnReset = unsafe extern "system" fn(
	this: IDirect3DDevice9,
	ppresentationparameters: *mut D3DPRESENT_PARAMETERS,
) -> HRESULT;

type FnCreateTexture = unsafe extern "system" fn(
	this: IDirect3DDevice9,
	width: u32,
	height: u32,
	levels: u32,
	usage: u32,
	format: D3DFORMAT,
	pool: D3DPOOL,
	pptexture: *mut Option<IDirect3DTexture9>,
	psharedhandle: *mut HANDLE,
) -> HRESULT;

type FnUpdateTexture = unsafe extern "system" fn(
	this: IDirect3DDevice9,
	pSourceTexture: *mut IDirect3DBaseTexture9,
	pDestinationTexture: *mut IDirect3DBaseTexture9,
) -> HRESULT;

type FnCreateQuery = unsafe extern "system" fn(
	this: IDirect3DDevice9,
	qtype: D3DQUERYTYPE,
	ppQuery: *mut *mut c_void,
) -> HRESULT;

static mut FN_ORG_ENDSCENE: Option<FnEndScene> = None;
static mut FN_ORG_PRESENT: Option<FnPresent> = None;
static mut FN_ORG_RESET: Option<FnReset> = None;
static mut FN_ORG_CREATE_TEXTURE: Option<FnCreateTexture> = None;
static mut FN_ORG_UPDATE_TEXTURE: Option<FnUpdateTexture> = None;
static mut FN_ORG_CREATE_QUERY: Option<FnCreateQuery> = None;

//TODO: move me to f_direct3d9device
unsafe extern "system" fn HOOK_EndScene(this: IDirect3DDevice9) -> HRESULT {
	let fn_EndScene = FN_ORG_ENDSCENE.unwrap();
	//let r = fn_EndScene(this);
	//log::trace!("HOOK_EndScene!");
	//r

	fn_EndScene(this)
}

//TODO: move me to f_direct3d9device
unsafe extern "system" fn HOOK_Present(
	this: IDirect3DDevice9,
	psourcerect: *const RECT,
	pdestrect: *const RECT,
	hdestwindowoverride: HWND,
	pdirtyregion: *const RGNDATA,
) -> HRESULT {
	#[cfg(feature = "gui")]
	crate::gui::console::draw(&this);

	let fn_Present = FN_ORG_PRESENT.unwrap();
	let r = fn_Present(
		this,
		psourcerect,
		pdestrect,
		hdestwindowoverride,
		pdirtyregion,
	);

	crate::api::blur_api::limit_fps();

	//log::trace!("HOOK_Present!");
	r
}

//TODO: move me to f_direct3d9device
unsafe extern "system" fn HOOK_Reset(
	this: IDirect3DDevice9,
	ppresentationparameters: *mut D3DPRESENT_PARAMETERS,
) -> HRESULT {
	#[cfg(feature = "gui")]
	crate::gui::console::reset();

	let fn_Reset = FN_ORG_RESET.unwrap();

	fn_Reset(this, ppresentationparameters)
}

unsafe extern "system" fn HOOK_CreateTexture(
	this: IDirect3DDevice9,
	width: u32,
	height: u32,
	levels: u32,
	usage: u32,
	format: D3DFORMAT,
	pool: D3DPOOL,
	pptexture: *mut Option<IDirect3DTexture9>,
	psharedhandle: *mut HANDLE,
) -> HRESULT {
	let fn_CreateTexture = FN_ORG_CREATE_TEXTURE.unwrap();
	let r = fn_CreateTexture(
		this,
		width,
		height,
		levels,
		usage,
		format,
		pool,
		pptexture,
		psharedhandle,
	);
	/*
		log::trace!(
			"fn_CreateTexture(
		width = {width:?},
		height = {height:?},
		levels = {levels:?},
		usage = {usage:?},
		format = {format:?},
		pool = {pool:?},
		pptexture = {pptexture:?},
		psharedhandle = {psharedhandle:?}
	)"
		);
		*/
	r
}

unsafe extern "system" fn HOOK_UpdateTexture(
	this: IDirect3DDevice9,
	pSourceTexture: *mut IDirect3DBaseTexture9,
	pDestinationTexture: *mut IDirect3DBaseTexture9,
) -> HRESULT {
	let fn_UpdateTexture = FN_ORG_UPDATE_TEXTURE.unwrap();
	// log::debug!("fn_UpdateTexture( _, pSourceTexture = {pSourceTexture:?}, pDestinationTexture = {pDestinationTexture:?})");
	let r = fn_UpdateTexture(this, pSourceTexture, pDestinationTexture);
	r
}

unsafe extern "system" fn HOOK_CreateQuery(
	this: IDirect3DDevice9,
	qtype: D3DQUERYTYPE,
	ppQuery: *mut *mut c_void,
) -> HRESULT {
	let fn_CreateQuery = FN_ORG_CREATE_QUERY.unwrap();
	let r = fn_CreateQuery(this, qtype, ppQuery);
	// log::debug!("fn_CreateQuery({ppQuery})")
	r
}

pub fn set_hook_endscene(dev: &IDirect3DDevice9) {
	let f = dev.vtable().EndScene;
	let fn_ptr: *mut c_void = f as *mut _;
	let fn_hook_ptr: *mut c_void = HOOK_EndScene as *mut _;
	let fn_saved: *mut *mut c_void = &mut std::ptr::null_mut();
	let v = unsafe { minhook_sys::MH_CreateHook(fn_ptr, fn_hook_ptr, fn_saved) };
	if v != minhook_sys::MH_OK {
		let v = v.to_string();
		panic!("MH_CreateHook(IDirect3DDevice9::EndScene) returned: {v}!");
	}
	unsafe {
		FN_ORG_ENDSCENE = Some(std::mem::transmute(*fn_saved));
	}
	let v = unsafe { minhook_sys::MH_EnableHook(fn_ptr) };
	if v != minhook_sys::MH_OK {
		panic!("MH_EnableHook(IDirect3DDevice9::EndScene) returned: {v}!");
	}
}

pub fn set_hook_present(dev: &IDirect3DDevice9) {
	let f = dev.vtable().Present;
	let fn_ptr: *mut c_void = f as *mut _;
	let fn_hook_ptr: *mut c_void = HOOK_Present as *mut _;
	let fn_saved: *mut *mut c_void = &mut std::ptr::null_mut();
	let v = unsafe { minhook_sys::MH_CreateHook(fn_ptr, fn_hook_ptr, fn_saved) };
	if v != minhook_sys::MH_OK {
		let v = v.to_string();
		panic!("MH_CreateHook(IDirect3DDevice9::Present) returned: {v}!");
	}
	unsafe {
		FN_ORG_PRESENT = Some(std::mem::transmute(*fn_saved));
	}
	let v = unsafe { minhook_sys::MH_EnableHook(fn_ptr) };
	if v != minhook_sys::MH_OK {
		panic!("MH_EnableHook(IDirect3DDevice9::Present) returned: {v}!");
	}
}

pub fn set_hook_reset(dev: &IDirect3DDevice9) {
	let f = dev.vtable().Reset;
	let fn_ptr: *mut c_void = f as *mut _;
	let fn_hook_ptr: *mut c_void = HOOK_Reset as *mut _;
	let fn_saved: *mut *mut c_void = &mut std::ptr::null_mut();
	let v = unsafe { minhook_sys::MH_CreateHook(fn_ptr, fn_hook_ptr, fn_saved) };
	if v != minhook_sys::MH_OK {
		let v = v.to_string();
		panic!("MH_CreateHook(IDirect3DDevice9::Reset) returned: {v}!");
	}
	unsafe {
		FN_ORG_RESET = Some(std::mem::transmute(*fn_saved));
	}
	let v = unsafe { minhook_sys::MH_EnableHook(fn_ptr) };
	if v != minhook_sys::MH_OK {
		panic!("MH_EnableHook(IDirect3DDevice9::Reset) returned: {v}!");
	}
}

pub fn set_hook_create_texture(dev: &IDirect3DDevice9) {
	let f = dev.vtable().CreateTexture;
	let fn_ptr: *mut c_void = f as *mut _;
	let fn_hook_ptr: *mut c_void = HOOK_CreateTexture as *mut _;
	let fn_saved: *mut *mut c_void = &mut std::ptr::null_mut();
	let v = unsafe { minhook_sys::MH_CreateHook(fn_ptr, fn_hook_ptr, fn_saved) };
	if v != minhook_sys::MH_OK {
		let v = v.to_string();
		panic!("MH_CreateHook(IDirect3DDevice9::CreateTexture) returned: {v}!");
	}
	unsafe {
		FN_ORG_CREATE_TEXTURE = Some(std::mem::transmute(*fn_saved));
	}
	let v = unsafe { minhook_sys::MH_EnableHook(fn_ptr) };
	if v != minhook_sys::MH_OK {
		panic!("MH_EnableHook(IDirect3DDevice9::CreateTexture) returned: {v}!");
	}
}

pub fn set_hook_update_texture(dev: &IDirect3DDevice9) {
	let f = dev.vtable().UpdateTexture;
	let fn_ptr: *mut c_void = f as *mut _;
	let fn_hook_ptr: *mut c_void = HOOK_UpdateTexture as *mut _;
	let fn_saved: *mut *mut c_void = &mut std::ptr::null_mut();
	let v = unsafe { minhook_sys::MH_CreateHook(fn_ptr, fn_hook_ptr, fn_saved) };
	if v != minhook_sys::MH_OK {
		let v = v.to_string();
		panic!("MH_CreateHook(IDirect3DDevice9::UpdateTexture) returned: {v}!");
	}
	unsafe {
		FN_ORG_UPDATE_TEXTURE = Some(std::mem::transmute(*fn_saved));
	}
	let v = unsafe { minhook_sys::MH_EnableHook(fn_ptr) };
	if v != minhook_sys::MH_OK {
		panic!("MH_EnableHook(IDirect3DDevice9::UpdateTexture) returned: {v}!");
	}
}

pub fn set_hook_create_query(dev: &IDirect3DDevice9) {
	let f = dev.vtable().CreateQuery;
	let fn_ptr: *mut c_void = f as *mut _;
	let fn_hook_ptr: *mut c_void = HOOK_CreateQuery as *mut _;
	let fn_saved: *mut *mut c_void = &mut std::ptr::null_mut();
	let v = unsafe { minhook_sys::MH_CreateHook(fn_ptr, fn_hook_ptr, fn_saved) };
	if v != minhook_sys::MH_OK {
		let v = v.to_string();
		panic!("MH_CreateHook(IDirect3DDevice9::CreateQuery) returned: {v}!");
	}
	unsafe {
		FN_ORG_CREATE_QUERY = Some(std::mem::transmute(*fn_saved));
	}
	let v = unsafe { minhook_sys::MH_EnableHook(fn_ptr) };
	if v != minhook_sys::MH_OK {
		panic!("MH_EnableHook(IDirect3DDevice9::CreateQuery) returned: {v}!");
	}
}
