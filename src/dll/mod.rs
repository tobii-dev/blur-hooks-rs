use windows::core::{HRESULT, Interface};

use windows::Win32::Graphics::Direct3D9::{IDirect3D9, IDirect3D9Ex};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

mod f_direct3d9;
use crate::dll::f_direct3d9::MyD3D9;

mod f_direct3d9device;

use log::info;

#[windows_dll::dll("C:\\Windows\\System32\\d3d9.dll")]
extern "system" {
    #[allow(non_snake_case)]
    pub unsafe extern "stdcall" fn D3DPERF_SetOptions(dw_options: u32);

    #[allow(non_snake_case)]
    pub unsafe extern "stdcall" fn Direct3DCreate9(sdk_version: u32) -> *mut IDirect3D9;

    #[allow(non_snake_case)]
    pub unsafe extern "stdcall" fn Direct3DCreate9Ex(
        sdk_version: u32,
        pp: *mut *mut IDirect3D9Ex,
    ) -> HRESULT;
}

#[no_mangle]
pub unsafe extern "stdcall" fn f_D3DPERF_SetOptions(dw_options: u32) {
    info!("f_D3DPERF_SetOptions(dw_options: (DWORD) {dw_options:?})");
    D3DPERF_SetOptions(dw_options)
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub unsafe extern "stdcall" fn f_Direct3DCreate9(sdk_version: u32) -> *mut IDirect3D9 {
    let r = Direct3DCreate9(sdk_version);
    info!("f_Direct3DCreate9(sdk_version: (UINT) {sdk_version:?}) -> *mut IDirect3D9 {r:?}");
    let m: IDirect3D9 = MyD3D9::new(r).into();
    m.into_raw() as _
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub unsafe extern "stdcall" fn f_Direct3DCreate9Ex(
    sdk_version: u32,
    pp: *mut *mut IDirect3D9Ex,
) -> HRESULT {
    let r = Direct3DCreate9Ex(sdk_version, pp);
    info!("f_Direct3DCreate9Ex(sdk_version: (UINT) {sdk_version:?}, pp: (*mut *mut IDirect3D9Ex) {pp:?}) -> HRESULT {r:?}");
    r
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(
    dll_module: windows::Win32::Foundation::HMODULE,
    call_reason: u32,
    _reserved: *mut std::ffi::c_void,
) -> i32 {
    match call_reason {
        DLL_PROCESS_ATTACH => crate::init(dll_module),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }
    true.into()
}
