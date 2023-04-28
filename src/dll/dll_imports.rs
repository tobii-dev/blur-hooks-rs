use windows::{
	core::HRESULT,
	Win32::Graphics::Direct3D9::{IDirect3D9, IDirect3D9Ex},
};

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
