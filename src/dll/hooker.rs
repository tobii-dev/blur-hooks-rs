#![allow(non_snake_case, non_camel_case_types)]
use std::ffi::c_void;
use std::ptr;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::Graphics::Direct3D9::D3DFORMAT;
use windows::Win32::Graphics::Direct3D9::D3DPOOL;
use windows::Win32::Graphics::Direct3D9::D3DPRESENT_PARAMETERS;
use windows::Win32::Graphics::Direct3D9::D3DQUERYTYPE;
use windows::Win32::Graphics::Direct3D9::IDirect3DBaseTexture9;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;
use windows::Win32::Graphics::Direct3D9::IDirect3DTexture9;

use windows::Win32::Graphics::Gdi::RGNDATA;

use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::RECT;

use windows::core::ComInterface;
use windows::core::HRESULT;
use windows::core::Interface;

type VoidPtr = *mut std::ffi::c_void;

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

type FnSetTexture = unsafe extern "system" fn(
	this: IDirect3DDevice9,
	stage: u32,
	pTexture: *mut IDirect3DBaseTexture9,
) -> HRESULT;

static mut FN_ORG_ENDSCENE: Option<FnEndScene> = None;
static mut FN_ORG_PRESENT: Option<FnPresent> = None;
static mut FN_ORG_RESET: Option<FnReset> = None;
static mut FN_ORG_CREATE_TEXTURE: Option<FnCreateTexture> = None;
static mut FN_ORG_UPDATE_TEXTURE: Option<FnUpdateTexture> = None;
static mut FN_ORG_CREATE_QUERY: Option<FnCreateQuery> = None;
static mut FN_ORG_SET_TEXTURE: Option<FnSetTexture> = None;

//TODO: move me to f_direct3d9device
unsafe extern "system" fn HOOK_EndScene(this: IDirect3DDevice9) -> HRESULT {
	let fn_EndScene = unsafe { FN_ORG_ENDSCENE.unwrap() };
	//let r = fn_EndScene(this);
	//log::trace!("HOOK_EndScene!");
	//r
	unsafe { fn_EndScene(this) }
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

	let fn_Present = unsafe { FN_ORG_PRESENT.unwrap() };
	let r = unsafe {
		fn_Present(
			this,
			psourcerect,
			pdestrect,
			hdestwindowoverride,
			pdirtyregion,
		)
	};

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

	let fn_Reset = unsafe { FN_ORG_RESET.unwrap() };

	unsafe { fn_Reset(this, ppresentationparameters) }
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
	let fn_CreateTexture = unsafe { FN_ORG_CREATE_TEXTURE.unwrap() };
	let r = unsafe {
		fn_CreateTexture(
			this,
			width,
			height,
			levels,
			usage,
			format,
			pool,
			pptexture,
			psharedhandle,
		)
	};
	if false {
		log::trace!(
			"fn_CreateTexture(width = {width:?}, height = {height:?}, levels = {levels:?}, usage = {usage:?}, format = {format:?}, pool = {pool:?}, pptexture = {pptexture:?}, psharedhandle = {psharedhandle:?}) -> {r}"
		);
	}
	r
}

unsafe extern "system" fn HOOK_UpdateTexture(
	this: IDirect3DDevice9,
	pSourceTexture: *mut IDirect3DBaseTexture9,
	pDestinationTexture: *mut IDirect3DBaseTexture9,
) -> HRESULT {
	let fn_UpdateTexture = unsafe { FN_ORG_UPDATE_TEXTURE.unwrap() };
	let r = unsafe { fn_UpdateTexture(this, pSourceTexture, pDestinationTexture) };
	if false {
		log::debug!(
			"fn_UpdateTexture( _, pSourceTexture = {pSourceTexture:?}, pDestinationTexture = {pDestinationTexture:?}) -> {r}"
		);
	}
	r
}

unsafe extern "system" fn HOOK_CreateQuery(
	this: IDirect3DDevice9,
	qtype: D3DQUERYTYPE,
	ppQuery: *mut *mut c_void,
) -> HRESULT {
	let fn_CreateQuery = unsafe { FN_ORG_CREATE_QUERY.unwrap() };
	let r = unsafe { fn_CreateQuery(this, qtype, ppQuery) };
	// log::debug!("fn_CreateQuery({ppQuery})")
	r
}

unsafe extern "system" fn HOOK_SetTexture(
	this: IDirect3DDevice9,
	stage: u32,
	pTexture: *mut IDirect3DBaseTexture9,
) -> HRESULT {
	let fn_SetTexture = unsafe { FN_ORG_SET_TEXTURE.unwrap() };
	if unsafe { this.TestCooperativeLevel() }.is_err() || pTexture.is_null() {
		return unsafe { fn_SetTexture(this, stage, pTexture) };
	}

	static mut ALPHA: Option<*mut IDirect3DTexture9> = None;
	unsafe {
		#[allow(static_mut_refs)]
		if ALPHA.is_none() {
			let width: u32 = 256;
			let height: u32 = 256;
			let levels: u32 = 1;
			let usage: u32 = 0;
			let format: D3DFORMAT = windows::Win32::Graphics::Direct3D9::D3DFMT_A8R8G8B8;
			let pool: D3DPOOL = windows::Win32::Graphics::Direct3D9::D3DPOOL_MANAGED;
			let mut q: *mut IDirect3DTexture9 = ptr::null_mut();
			let pptexture: *mut *mut IDirect3DTexture9 = &mut q as *mut _;
			let psharedhandle: *mut HANDLE = ptr::null_mut();
			this.CreateTexture(
				width,
				height,
				levels,
				usage,
				format,
				pool,
				pptexture.cast(),
				psharedhandle,
			)
			.unwrap();
			ALPHA = Some(pptexture.read())
		}
	}

	#[derive(Debug)]
	struct MySavedData {
		hash: Option<u32>,
		cooler_tex: *mut IDirect3DBaseTexture9,
	}
	impl MySavedData {
		fn extract(
			pTexture: *mut IDirect3DBaseTexture9,
			cooler_tex: *mut IDirect3DBaseTexture9,
		) -> Option<Self> {
			let p: *mut std::ffi::c_void = pTexture as _;
			let tex: &IDirect3DBaseTexture9 =
				unsafe { IDirect3DBaseTexture9::from_raw_borrowed(&p).unwrap() };
			if unsafe { tex.GetType() } != windows::Win32::Graphics::Direct3D9::D3DRTYPE_TEXTURE {
				return None;
			}
			let Ok(tex) = tex.cast::<IDirect3DTexture9>() else {
				return None;
			};
			let mut my_data = Self {
				hash: None,
				cooler_tex: ptr::null_mut(),
			};
			let mut psizeofdata: u32 = size_of_val(&my_data).try_into().unwrap();
			if unsafe {
				tex.GetPrivateData(
					&IDirect3DTexture9::IID as _,
					&mut my_data as *mut MySavedData as *mut core::ffi::c_void,
					&mut psizeofdata as *mut u32,
				)
				.is_ok()
			} {
				// log::debug!("reused!");
				return Some(my_data);
			}

			fn get_tex_hash(tex: &IDirect3DTexture9) -> windows::core::Result<u32> {
				let desc = unsafe {
					let mut pdesc: windows::Win32::Graphics::Direct3D9::D3DSURFACE_DESC =
						windows::Win32::Graphics::Direct3D9::D3DSURFACE_DESC::default();
					tex.GetLevelDesc(0, &mut pdesc as _)?;
					pdesc
				};
				if desc.Pool == windows::Win32::Graphics::Direct3D9::D3DPOOL_DEFAULT {
					return Err(windows::core::Error::OK);
				}
				use windows::Win32::Graphics::Direct3D9 as D;
				// https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dformat
				let bits: Option<u32> = match desc.Format {
					D::D3DFMT_DXT1 => Some(4),

					D::D3DFMT_A8
					| D::D3DFMT_L8
					| D::D3DFMT_P8
					| D::D3DFMT_R3G3B2
					| D::D3DFMT_S8_LOCKABLE => Some(8),

					D::D3DFMT_A1R5G5B5
					| D::D3DFMT_A4L4
					| D::D3DFMT_A4R4G4B4
					| D::D3DFMT_A8L8
					| D::D3DFMT_A8P8
					| D::D3DFMT_A8R3G3B2
					| D::D3DFMT_CxV8U8
					| D::D3DFMT_D15S1
					| D::D3DFMT_D16
					| D::D3DFMT_D16_LOCKABLE
					| D::D3DFMT_G8R8_G8B8
					| D::D3DFMT_INDEX16
					| D::D3DFMT_L16
					| D::D3DFMT_L6V5U5
					| D::D3DFMT_R16F
					| D::D3DFMT_R5G6B5
					| D::D3DFMT_R8G8_B8G8
					| D::D3DFMT_UYVY
					| D::D3DFMT_V8U8
					| D::D3DFMT_X1R5G5B5
					| D::D3DFMT_X4R4G4B4
					| D::D3DFMT_YUY2 => Some(16),

					D::D3DFMT_R8G8B8 => Some(24),

					D::D3DFMT_A2B10G10R10
					| D::D3DFMT_A2B10G10R10_XR_BIAS
					| D::D3DFMT_A2R10G10B10
					| D::D3DFMT_A2W10V10U10
					| D::D3DFMT_A8B8G8R8
					| D::D3DFMT_A8R8G8B8
					| D::D3DFMT_D24FS8
					| D::D3DFMT_D24S8
					| D::D3DFMT_D24X4S4
					| D::D3DFMT_D24X8
					| D::D3DFMT_D32
					| D::D3DFMT_D32F_LOCKABLE
					| D::D3DFMT_D32_LOCKABLE
					| D::D3DFMT_G16R16
					| D::D3DFMT_G16R16F
					| D::D3DFMT_INDEX32
					| D::D3DFMT_MULTI2_ARGB8
					| D::D3DFMT_Q8W8V8U8
					| D::D3DFMT_R32F
					| D::D3DFMT_V16U16
					| D::D3DFMT_X8B8G8R8
					| D::D3DFMT_X8L8V8U8
					| D::D3DFMT_X8R8G8B8 => Some(32),

					D::D3DFMT_A16B16G16R16
					| D::D3DFMT_A16B16G16R16F
					| D::D3DFMT_G32R32F
					| D::D3DFMT_Q16W16V16U16 => Some(64),

					D::D3DFMT_A32B32G32R32F => Some(128),

					D::D3DFMT_DXT2 | D::D3DFMT_DXT3 | D::D3DFMT_DXT4 | D::D3DFMT_DXT5 => Some(8),

					D::D3DFMT_BINARYBUFFER | D::D3DFMT_VERTEXDATA | D::D3DFMT_UNKNOWN => None,
					_ => None,
				};
				let bits = bits.unwrap();

				let rect = unsafe {
					// https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dtexture9-lockrect#remarks
					let mut rect: windows::Win32::Graphics::Direct3D9::D3DLOCKED_RECT =
						windows::Win32::Graphics::Direct3D9::D3DLOCKED_RECT::default();
					tex.LockRect(
						0,
						&mut rect as _,
						core::ptr::null(),
						windows::Win32::Graphics::Direct3D9::D3DLOCK_READONLY.cast_unsigned(),
					)
					.unwrap();
					// log::debug!("pBits = {}", rect.pBits as usize);
					rect
				};
				let size = (bits * desc.Width * desc.Height) / 8;

				// // More stolen goods from texmod:

				//  * BIG THANKS TO RS !!
				//  *
				//  * who gave me his hashing algorithm (well or crc32 algorithm^^)
				//  *
				// The hash function is CRC32 using polynomial 0xEDB88320.
				// However, the hashed data is calculated incorrectly in TexMod: it's simply BytesPerPixel * Width * Height, from the beginning of the data (that is mapped using LockRect).
				// The problem is that it doesn't take the pitch into account and BytesPerPixel may be wrong for some rare formats (not sure about that).
				// */
				//

				// ```c
				// #define CRC32POLY 0xEDB88320u /* CRC-32 Polynom */
				// #define CRC32POLY 0xEDB88320u /* CRC-32 Polynom */
				// #define ulCrc_in 0xffffffff
				//
				// unsigned int GetCRC32(char* pcDatabuf, unsigned int ulDatalen) {
				// 	unsigned int crc = ulCrc_in;
				// 	for (unsigned int idx = 0u; idx < ulDatalen; idx++) {
				// 		unsigned int data = *pcDatabuf++;
				// 		for (unsigned int bit = 0u; bit < 8u; bit++, data >>= 1) {
				// 			crc = (crc >> 1) ^ (((crc ^ data) & 1) ? CRC32POLY : 0);
				// 		}
				// 	}
				// 	return (crc);
				// }
				// ```

				fn getCRC32(buf: *const u8, buflen: usize) -> u32 {
					/// CRC-32 Polynom
					const CRC32POLY: u32 = 0xEDB88320;
					const CRC32_IN: u32 = 0xFFFFFFFF;

					let mut crc: u32 = CRC32_IN;
					for idx in 0..buflen {
						let mut data = unsafe { buf.byte_add(idx).read() };
						for _ in 0..8 {
							crc = (crc >> 1)
								^ if ((crc ^ (data as u32)) & 1) != 0 {
									CRC32POLY
								} else {
									0
								};
							data >>= 1;
						}
					}
					crc
				}
				let crc = getCRC32(rect.pBits as *mut u8, size.try_into().unwrap());
				unsafe { tex.UnlockRect(0).unwrap() };
				Ok(crc)
			}

			my_data.hash = get_tex_hash(&tex).ok();
			if let Some(hash) = my_data.hash {
				log::debug!("0x{hash:08X}");
				if hash == 0xF6FA7B91 {
					my_data.cooler_tex = cooler_tex;
				}
			}
			unsafe {
				tex.SetPrivateData(
					&IDirect3DTexture9::IID as _,
					&mut my_data as *mut MySavedData as *mut core::ffi::c_void,
					size_of_val(&my_data).try_into().unwrap(),
					0,
				)
				.unwrap();
			};
			Some(my_data)
		}
	}
	#[allow(static_mut_refs)]
	let cooler_tex = unsafe { ALPHA.unwrap() };

	let Some(saved_data) = MySavedData::extract(pTexture, cooler_tex.cast()) else {
		return unsafe { fn_SetTexture(this, stage, pTexture) };
	};
	if saved_data.cooler_tex.is_null() {
		return unsafe { fn_SetTexture(this, stage, pTexture) };
	}
	// log::debug!("fn_SetTexture(_, stage = {stage}, pTexture = {pTexture:?})");
	unsafe { fn_SetTexture(this, stage, saved_data.cooler_tex) }
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
		FN_ORG_ENDSCENE = Some(std::mem::transmute::<VoidPtr, FnEndScene>(*fn_saved));
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
		FN_ORG_PRESENT = Some(std::mem::transmute::<VoidPtr, FnPresent>(*fn_saved));
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
		FN_ORG_RESET = Some(std::mem::transmute::<VoidPtr, FnReset>(*fn_saved));
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
		FN_ORG_CREATE_TEXTURE = Some(std::mem::transmute::<VoidPtr, FnCreateTexture>(*fn_saved));
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
		FN_ORG_UPDATE_TEXTURE = Some(std::mem::transmute::<VoidPtr, FnUpdateTexture>(*fn_saved));
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
		FN_ORG_CREATE_QUERY = Some(std::mem::transmute::<VoidPtr, FnCreateQuery>(*fn_saved));
	}
	let v = unsafe { minhook_sys::MH_EnableHook(fn_ptr) };
	if v != minhook_sys::MH_OK {
		panic!("MH_EnableHook(IDirect3DDevice9::CreateQuery) returned: {v}!");
	}
}

pub fn set_hook_set_texture(dev: &IDirect3DDevice9) {
	let f = dev.vtable().SetTexture;
	let fn_ptr: *mut c_void = f as *mut _;
	let fn_hook_ptr: *mut c_void = HOOK_SetTexture as *mut _;
	let fn_saved: *mut *mut c_void = &mut std::ptr::null_mut();
	let v = unsafe { minhook_sys::MH_CreateHook(fn_ptr, fn_hook_ptr, fn_saved) };
	if v != minhook_sys::MH_OK {
		let v = v.to_string();
		panic!("MH_CreateHook(IDirect3DDevice9::SetTexture) returned: {v}!");
	}
	unsafe {
		FN_ORG_SET_TEXTURE = Some(std::mem::transmute::<VoidPtr, FnSetTexture>(*fn_saved));
	}
	let v = unsafe { minhook_sys::MH_EnableHook(fn_ptr) };
	if v != minhook_sys::MH_OK {
		panic!("MH_EnableHook(IDirect3DDevice9::SetTexture) returned: {v}!");
	}
}
