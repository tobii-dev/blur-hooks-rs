use windows::core::{Interface, HRESULT};

use windows::Win32::Graphics::Direct3D9::{IDirect3D9, IDirect3D9Ex};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use crate::dll::f_direct3d9::MyD3D9;

#[no_mangle]
pub unsafe extern "stdcall" fn f_D3DPERF_SetOptions(dw_options: u32) {
	log::trace!("f_D3DPERF_SetOptions(dw_options: (DWORD) {dw_options:?})");
	crate::dll::dll_imports::D3DPERF_SetOptions(dw_options)
}

#[no_mangle]
pub unsafe extern "stdcall" fn f_Direct3DCreate9(sdk_version: u32) -> *mut IDirect3D9 {
	let r = crate::dll::dll_imports::Direct3DCreate9(sdk_version);
	log::trace!("f_Direct3DCreate9(sdk_version: (UINT) {sdk_version:?}) -> *mut IDirect3D9 {r:?}");
	let m: IDirect3D9 = MyD3D9::new(r).into();
	m.into_raw() as _
}

#[no_mangle]
pub unsafe extern "stdcall" fn f_Direct3DCreate9Ex(
	sdk_version: u32,
	pp: *mut *mut IDirect3D9Ex,
) -> HRESULT {
	let r = crate::dll::dll_imports::Direct3DCreate9Ex(sdk_version, pp);
	log::trace!("f_Direct3DCreate9Ex(sdk_version: (UINT) {sdk_version:?}, pp: (*mut *mut IDirect3D9Ex) {pp:?}) -> HRESULT {r:?}");
	r
}

#[no_mangle]
extern "system" fn DllMain(
	dll_module: windows::Win32::Foundation::HMODULE,
	call_reason: u32,
	_reserved: *mut std::ffi::c_void,
) -> i32 {
	match call_reason {
		DLL_PROCESS_ATTACH => super::core::init(dll_module),
		DLL_PROCESS_DETACH => super::core::free(dll_module),
		_ => (),
	}
	true.into()
}
