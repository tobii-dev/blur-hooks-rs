// TODO: Credit TexMod with links: get_crc32(), ...
// TODO: Get rid of static_mut_refs
// TODO: Clean up GetPrivateData() extracts
// TODO: Error handling cleanup
// TODO: Certain textures: https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dtexture9#remarks

use std::collections::BTreeMap;

use image::GenericImageView;
use windows::{
	Win32::Graphics::Direct3D9::{
		D3DFMT_A8R8G8B8, D3DLOCKED_RECT, D3DPOOL_MANAGED, IDirect3DBaseTexture9, IDirect3DDevice9,
		IDirect3DTexture9,
	},
	core::{ComInterface, Interface},
};

pub struct TexMgr {
	map: BTreeMap<u32, *mut IDirect3DBaseTexture9>,
}

pub static mut TEXMGR: Option<TexMgr> = None;
pub fn init(dev: &IDirect3DDevice9) {
	unsafe {
		#[allow(static_mut_refs)]
		TEXMGR.replace(TexMgr::init(dev));
	}
}

impl TexMgr {
	fn init(dev: &IDirect3DDevice9) -> Self {
		let mut this = Self {
			map: BTreeMap::new(),
		};
		let path_texs_dir = std::path::Path::new(".")
			.join("amax")
			.join("gfx")
			.join("tex");
		let path_display = path_texs_dir.display();
		log::info!("Loading Textures from: {path_display}");
		let entries = match path_texs_dir.read_dir() {
			Ok(p) => p,
			Err(e) => {
				log::error!("[{path_display}] .read_dir() failed: {e} - Does the directory exist?");
				return this;
			}
		};
		for entry in entries.filter_map(|e| e.map(|e| e.path()).ok()) {
			if !entry
				.extension()
				.and_then(|ext| ext.to_str())
				.is_some_and(|ext| ext == "png")
			{
				continue;
			};
			let Some(name) = entry.file_prefix().and_then(|name| name.to_str()) else {
				continue;
			};
			if name.len() != "FFFFFFFF".len() {
				continue;
			}
			// if !name.chars().all(|c| c.is_ascii_hexdigit()) { continue; }
			let Ok(hash) = u32::from_str_radix(name, 16) else {
				continue;
			};
			let entry_display = &entry.display();
			log::info!("Loading: 0x{hash:08X} <- {entry_display}");
			let tex = load_img(dev, entry.as_path());
			this.map.insert(hash, tex.cast());
		}
		log::info!("Done loading Textures");
		this
	}

	pub fn update(
		&mut self,
		dev: &IDirect3DDevice9,
		p: *mut IDirect3DBaseTexture9,
	) -> *mut IDirect3DBaseTexture9 {
		if unsafe { dev.TestCooperativeLevel() }.is_err() || p.is_null() {
			return p;
		}
		match MySavedData::update(self, p) {
			Some(MySavedData {
				hash: Some(_hash),
				cooler_tex,
			}) => {
				if cooler_tex.is_null() {
					p
				} else {
					// log::info!("Texture update: 0x{_hash:08X}");
					cooler_tex
				}
			}
			_ => p,
		}
	}
}

fn load_img(dev: &IDirect3DDevice9, img_path: &std::path::Path) -> *mut IDirect3DTexture9 {
	let buf = std::io::BufReader::new(std::fs::File::open(img_path).unwrap());
	let img = image::load(buf, image::ImageFormat::Png).unwrap();
	let (width, height) = img.dimensions();
	let src: Vec<_> = img
		.pixels()
		.map(|(_x, _y, p)| p.0)
		.map(|[r, g, b, a]| PixelBGRA { r, g, b, a })
		.collect();
	d3d9_create_tex_from_mem(dev, &src, width, height)
}

// TODO: DDS files
// D3DFMT_A8R8G8B8 requires the texture be stored this way
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct PixelBGRA {
	b: u8, // blue
	g: u8, // green
	r: u8, // red
	a: u8, // alpha
}

fn d3d9_create_tex_from_mem(
	dev: &IDirect3DDevice9,
	src: &[PixelBGRA],
	width: u32,
	height: u32,
) -> *mut IDirect3DTexture9 {
	let mut tex_ptr: Option<IDirect3DTexture9> = None;
	unsafe {
		// https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createtexture
		// https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dformat
		// https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dpool
		let r = dev.CreateTexture(
			width,
			height,
			1,
			0u32,
			D3DFMT_A8R8G8B8,
			D3DPOOL_MANAGED, // D3DPOOL_MANAGED allows this texture to survive dev.Reset(..)
			&mut tex_ptr,
			std::ptr::null_mut(),
		);
		// log::trace!("dev.CreateTexture(tex_ptr: {tex_ptr:?}) -> {r:?}");
		r.expect("dev.CreateTexture failed");
	};
	let tex_ptr = tex_ptr.expect("dev.CreateTexture returned null tex ptr");

	// Then get to writing the inner texture pixel data with tex.LockRect()
	unsafe {
		let mut rect: D3DLOCKED_RECT = D3DLOCKED_RECT::default();
		tex_ptr
			.LockRect(0, &mut rect, std::ptr::null_mut(), 0)
			.expect("tex_ptr.LockRect(..) failed");
		assert!(width * height == src.len() as u32);
		let dst: &mut [PixelBGRA] =
			std::slice::from_raw_parts_mut(rect.pBits as *mut PixelBGRA, src.len());
		dst.copy_from_slice(src);
		tex_ptr
			.UnlockRect(0)
			.expect("tex_ptr.UnlockRect(..) failed"); // UPLOAD IT
	}
	// log::trace!("Created IDirect3DTexture9: {tex_ptr:?}");
	tex_ptr.into_raw() as *mut IDirect3DTexture9
	// .into_raw() prevents the texture getting cleared by mem::drop().
	// Only the d3d9 device knows about it now
}

#[derive(Debug)]
struct MySavedData {
	hash: Option<u32>,
	cooler_tex: *mut IDirect3DBaseTexture9,
}
impl MySavedData {
	fn update(mgr: &TexMgr, ptex: *mut IDirect3DBaseTexture9) -> Option<Self> {
		let tex: *mut std::ffi::c_void = ptex as _;
		let tex: &IDirect3DBaseTexture9 =
			unsafe { IDirect3DBaseTexture9::from_raw_borrowed(&tex).unwrap() };
		if unsafe { tex.GetType() } != windows::Win32::Graphics::Direct3D9::D3DRTYPE_TEXTURE {
			return None;
		}
		let Ok(tex) = tex.cast::<IDirect3DTexture9>() else {
			return None;
		};
		let mut my_data = Self {
			hash: None,
			cooler_tex: std::ptr::null_mut(),
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
			return Some(my_data);
		}

		// TODO: Clean up error handling
		// TODO: Handle D3DPOOL_DEFAULT
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

			fn get_crc32(buf: *const u8, buflen: usize) -> u32 {
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
			let crc = get_crc32(rect.pBits as *mut u8, size.try_into().unwrap());
			unsafe { tex.UnlockRect(0).unwrap() };
			Ok(crc)
		}

		my_data.hash = get_tex_hash(&tex).ok();
		if let Some(hash) = my_data.hash {
			// log::debug!("0x{hash:08X}");
			if let Some(cooler) = mgr.map.get(&hash) {
				my_data.cooler_tex = *cooler;
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
