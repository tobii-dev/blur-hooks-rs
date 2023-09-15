#![allow(non_snake_case, non_camel_case_types)]
use std::ffi::c_void;

use windows::Win32::Graphics::Direct3D9::IDirect3D9;
use windows::Win32::Graphics::Direct3D9::IDirect3D9_Impl;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;
use windows::Win32::Graphics::Direct3D9::D3DADAPTER_IDENTIFIER9;
use windows::Win32::Graphics::Direct3D9::D3DCAPS9;
use windows::Win32::Graphics::Direct3D9::D3DDEVTYPE;
use windows::Win32::Graphics::Direct3D9::D3DDISPLAYMODE;
use windows::Win32::Graphics::Direct3D9::D3DFORMAT;
use windows::Win32::Graphics::Direct3D9::D3DMULTISAMPLE_TYPE;
use windows::Win32::Graphics::Direct3D9::D3DPRESENT_PARAMETERS;
use windows::Win32::Graphics::Direct3D9::D3DRESOURCETYPE;

use windows::Win32::Graphics::Gdi::RGNDATA;

use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::RECT;

use windows::core::implement;
use windows::core::Interface;
use windows::core::HRESULT;

use log::{info, trace};

use crate::dll::fps::limit_fps;

#[derive(Debug)]
#[implement(IDirect3D9)]
pub struct MyD3D9 {
	f: IDirect3D9,
}

//TODO: because these methods "really" actually belong to IDirect3DDevice9;
//they should be defined in crate::dll::f_direct3d9device

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

static mut FN_ORG_ENDSCENE: Option<FnEndScene> = None;
static mut FN_ORG_PRESENT: Option<FnPresent> = None;
static mut FN_ORG_RESET: Option<FnReset> = None;

//TODO: move me to f_direct3d9device
unsafe extern "system" fn HOOK_EndScene(this: IDirect3DDevice9) -> HRESULT {
	let fn_EndScene = FN_ORG_ENDSCENE.unwrap();
	//let r = fn_EndScene(this);
	//trace!("HOOK_EndScene!");
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
	{
		use windows::Win32::Graphics::Direct3D9::D3DDEVICE_CREATION_PARAMETERS;
		let pparameters = &mut D3DDEVICE_CREATION_PARAMETERS::default();
		match this.GetCreationParameters(pparameters) {
			Ok(_) => {
				let hwnd = pparameters.hFocusWindow;
				crate::gui::console::draw(&this, hwnd);
			}
			Err(err) => {
				log::warn!("GetCreationParameters() returned {err}");
			}
		}
	}

	let fn_Present = FN_ORG_PRESENT.unwrap();
	let r = fn_Present(
		this,
		psourcerect,
		pdestrect,
		hdestwindowoverride,
		pdirtyregion,
	);

	let fps = crate::api::blur_api::BLUR_API.fps;
	if fps > 0.0 {
		limit_fps(fps);
	}
	//trace!("HOOK_Present!");
	r
}

//TODO: move me to f_direct3d9device
unsafe extern "system" fn HOOK_Reset(
	this: IDirect3DDevice9,
	ppresentationparameters: *mut D3DPRESENT_PARAMETERS,
) -> HRESULT {
	trace!("HOOK_Reset_A!");
	crate::gui::console::reset();
	trace!("HOOK_Reset_B!");
	let fn_Reset = FN_ORG_RESET.unwrap();
	let r = fn_Reset(this, ppresentationparameters);
	trace!("HOOK_Reset_C!");
	r
}

impl MyD3D9 {
	pub fn new(f: *mut IDirect3D9) -> Self {
		let f = unsafe { IDirect3D9::from_raw(f as _) };
		let r = MyD3D9 { f };
		trace!("MyD3D9::new() -> {r:#?}");
		r
	}
}

impl IDirect3D9_Impl for MyD3D9 {
	fn RegisterSoftwareDevice(
		&self,
		pinitializefunction: *mut core::ffi::c_void,
	) -> windows::core::Result<()> {
		info!("MyD3D9::RegisterSoftwareDevice_pre");
		let r = unsafe { self.f.RegisterSoftwareDevice(pinitializefunction) };
		info!("MyD3D9::RegisterSoftwareDevice");
		r
	}

	fn GetAdapterCount(&self) -> u32 {
		info!("MyD3D9::GetAdapterCount_pre");
		let r = unsafe { self.f.GetAdapterCount() };
		info!("MyD3D9::GetAdapterCount");
		r
	}

	fn GetAdapterIdentifier(
		&self,
		adapter: u32,
		flags: u32,
		pidentifier: *mut D3DADAPTER_IDENTIFIER9,
	) -> windows::core::Result<()> {
		info!("MyD3D9::GetAdapterIdentifier_pre");
		let r = unsafe { self.f.GetAdapterIdentifier(adapter, flags, pidentifier) };
		info!("MyD3D9::GetAdapterIdentifier");
		r
	}

	fn GetAdapterModeCount(&self, adapter: u32, format: D3DFORMAT) -> u32 {
		info!("MyD3D9::GetAdapterModeCount_pre");
		let r = unsafe { self.f.GetAdapterModeCount(adapter, format) };
		info!("MyD3D9::GetAdapterModeCount");
		r
	}

	fn EnumAdapterModes(
		&self,
		adapter: u32,
		format: D3DFORMAT,
		mode: u32,
		pmode: *mut D3DDISPLAYMODE,
	) -> windows::core::Result<()> {
		info!("MyD3D9::EnumAdapterModes_pre");
		let r = unsafe { self.f.EnumAdapterModes(adapter, format, mode, pmode) };
		info!("MyD3D9::EnumAdapterModes");
		r
	}

	fn GetAdapterDisplayMode(
		&self,
		adapter: u32,
		pmode: *mut D3DDISPLAYMODE,
	) -> windows::core::Result<()> {
		info!("MyD3D9::GetAdapterDisplayMode_pre");
		let r = unsafe { self.f.GetAdapterDisplayMode(adapter, pmode) };
		info!("MyD3D9::GetAdapterDisplayMode");
		r
	}

	fn CheckDeviceType(
		&self,
		adapter: u32,
		devtype: D3DDEVTYPE,
		adapterformat: D3DFORMAT,
		backbufferformat: D3DFORMAT,
		bwindowed: windows::Win32::Foundation::BOOL,
	) -> windows::core::Result<()> {
		info!("MyD3D9::CheckDeviceType_pre");
		let r = unsafe {
			self.f
				.CheckDeviceType(adapter, devtype, adapterformat, backbufferformat, bwindowed)
		};
		info!("MyD3D9::CheckDeviceType");
		r
	}

	fn CheckDeviceFormat(
		&self,
		adapter: u32,
		devicetype: D3DDEVTYPE,
		adapterformat: D3DFORMAT,
		usage: u32,
		rtype: D3DRESOURCETYPE,
		checkformat: D3DFORMAT,
	) -> windows::core::Result<()> {
		info!("MyD3D9::CheckDeviceFormat_pre");
		let r = unsafe {
			self.f.CheckDeviceFormat(
				adapter,
				devicetype,
				adapterformat,
				usage,
				rtype,
				checkformat,
			)
		};
		info!("MyD3D9::CheckDeviceFormat");
		r
	}

	fn CheckDeviceMultiSampleType(
		&self,
		adapter: u32,
		devicetype: D3DDEVTYPE,
		surfaceformat: D3DFORMAT,
		windowed: windows::Win32::Foundation::BOOL,
		multisampletype: D3DMULTISAMPLE_TYPE,
		pqualitylevels: *mut u32,
	) -> windows::core::Result<()> {
		info!("MyD3D9::CheckDeviceMultiSampleType_pre");
		let r = unsafe {
			self.f.CheckDeviceMultiSampleType(
				adapter,
				devicetype,
				surfaceformat,
				windowed,
				multisampletype,
				pqualitylevels,
			)
		};
		info!("MyD3D9::CheckDeviceMultiSampleType");
		r
	}

	fn CheckDepthStencilMatch(
		&self,
		adapter: u32,
		devicetype: D3DDEVTYPE,
		adapterformat: D3DFORMAT,
		rendertargetformat: D3DFORMAT,
		depthstencilformat: D3DFORMAT,
	) -> windows::core::Result<()> {
		info!("MyD3D9::CheckDepthStencilMatch_pre");
		let r = unsafe {
			self.f.CheckDepthStencilMatch(
				adapter,
				devicetype,
				adapterformat,
				rendertargetformat,
				depthstencilformat,
			)
		};
		info!("MyD3D9::CheckDepthStencilMatch");
		r
	}

	fn CheckDeviceFormatConversion(
		&self,
		adapter: u32,
		devicetype: D3DDEVTYPE,
		sourceformat: D3DFORMAT,
		targetformat: D3DFORMAT,
	) -> windows::core::Result<()> {
		info!("MyD3D9::CheckDeviceFormatConversion_pre");
		let r = unsafe {
			self.f
				.CheckDeviceFormatConversion(adapter, devicetype, sourceformat, targetformat)
		};
		info!("MyD3D9::CheckDeviceFormatConversion");
		r
	}

	fn GetDeviceCaps(
		&self,
		adapter: u32,
		devicetype: D3DDEVTYPE,
		pcaps: *mut D3DCAPS9,
	) -> windows::core::Result<()> {
		info!("MyD3D9::GetDeviceCaps_pre");
		let r = unsafe { self.f.GetDeviceCaps(adapter, devicetype, pcaps) };
		info!("MyD3D9::GetDeviceCaps");
		r
	}

	fn GetAdapterMonitor(&self, adapter: u32) -> windows::Win32::Graphics::Gdi::HMONITOR {
		info!("MyD3D9::GetAdapterMonitor_pre");
		let r = unsafe { self.f.GetAdapterMonitor(adapter) };
		info!("MyD3D9::GetAdapterMonitor");
		r
	}

	fn CreateDevice(
		&self,
		adapter: u32,
		devicetype: D3DDEVTYPE,
		hfocuswindow: HWND,
		behaviorflags: u32,
		ppresentationparameters: *mut D3DPRESENT_PARAMETERS,
		ppreturneddeviceinterface: *mut Option<IDirect3DDevice9>,
	) -> windows::core::Result<()> {
		let r = unsafe {
			self.f.CreateDevice(
				adapter,
				devicetype,
				hfocuswindow,
				behaviorflags,
				ppresentationparameters,
				ppreturneddeviceinterface,
			)
		};

		let dev = unsafe { ppreturneddeviceinterface.as_ref() }
			.unwrap()
			.to_owned()
			.unwrap();
		set_hook_endscene(&dev);
		set_hook_present(&dev);
		set_hook_reset(&dev);

		//unsafe { ppreturneddeviceinterface.write(Some(crate::dll::f_direct3d9device::MyDirect3DDevice9::new(dev).into())) };
		r
	}
}

fn set_hook_endscene(dev: &IDirect3DDevice9) {
	let f = dev.vtable().EndScene;
	//let fn_ptr: FnEndScene = unsafe { std::mem::transmute(f) };
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

fn set_hook_present(dev: &IDirect3DDevice9) {
	let f = dev.vtable().Present;
	//let fn_ptr: FnPresent = unsafe { std::mem::transmute(f) };
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

fn set_hook_reset(dev: &IDirect3DDevice9) {
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
