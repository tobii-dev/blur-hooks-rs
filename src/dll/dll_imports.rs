use windows::{
	core::HRESULT,
	Win32::Graphics::Direct3D9::{IDirect3D9, IDirect3D9Ex},
};

#[windows_dll::dll("C:\\Windows\\System32\\d3d9.dll")]
extern "system" {
	#[allow(non_snake_case)]
	#[link_name = "D3DPERF_SetOptions"]
	pub unsafe extern "stdcall" fn _D3DPERF_SetOptions(dw_options: u32);

	#[allow(non_snake_case)]
	#[link_name = "Direct3DCreate9"]
	pub unsafe extern "stdcall" fn _Direct3DCreate9(sdk_version: u32) -> *mut IDirect3D9;

	#[allow(non_snake_case)]
	#[link_name = "Direct3DCreate9Ex"]
	pub unsafe extern "stdcall" fn _Direct3DCreate9Ex(
		sdk_version: u32,
		pp: *mut *mut IDirect3D9Ex,
	) -> HRESULT;
}

#[windows_dll::dll("amax\\dxvk.dll")]
extern "system" {
	#[allow(non_snake_case)]
	#[link_name = "D3DPERF_SetOptions"]
	unsafe extern "stdcall" fn dxvk_D3DPERF_SetOptions(dw_options: u32);

	#[allow(non_snake_case)]
	#[link_name = "Direct3DCreate9"]
	unsafe extern "stdcall" fn dxvk_Direct3DCreate9(sdk_version: u32) -> *mut IDirect3D9;

	#[allow(non_snake_case)]
	#[link_name = "Direct3DCreate9Ex"]
	unsafe extern "stdcall" fn dxvk_Direct3DCreate9Ex(
		sdk_version: u32,
		pp: *mut *mut IDirect3D9Ex,
	) -> HRESULT;
}

#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn D3DPERF_SetOptions(dw_options: u32) {
	if dxvk_Direct3DCreate9::exists() {
		return dxvk_D3DPERF_SetOptions(dw_options);
	};
	_D3DPERF_SetOptions(dw_options)
}

#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn Direct3DCreate9(sdk_version: u32) -> *mut IDirect3D9 {
	if dxvk_Direct3DCreate9::exists() {
		return dxvk_Direct3DCreate9(sdk_version);
	};
	_Direct3DCreate9(sdk_version)
}

#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn Direct3DCreate9Ex(
	sdk_version: u32,
	pp: *mut *mut IDirect3D9Ex,
) -> HRESULT {
	if dxvk_Direct3DCreate9Ex::exists() {
		return dxvk_Direct3DCreate9Ex(sdk_version, pp);
	};
	_Direct3DCreate9Ex(sdk_version, pp)
}
