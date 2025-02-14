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

use windows::Win32::Foundation::HWND;

use windows::core::implement;
use windows::core::Interface;

#[derive(Debug)]
#[implement(IDirect3D9)]
pub struct MyD3D9 {
	f: IDirect3D9,
}

impl MyD3D9 {
	pub fn new(f: *mut IDirect3D9) -> Self {
		let f = unsafe { IDirect3D9::from_raw(f as _) };
		let r = MyD3D9 { f };
		log::info!("MyD3D9::new() -> {r:#?}");
		r
	}
}

impl IDirect3D9_Impl for MyD3D9 {
	fn RegisterSoftwareDevice(
		&self,
		pinitializefunction: *mut core::ffi::c_void,
	) -> windows::core::Result<()> {
		log::trace!("MyD3D9::RegisterSoftwareDevice_pre");
		let r = unsafe { self.f.RegisterSoftwareDevice(pinitializefunction) };
		log::trace!("MyD3D9::RegisterSoftwareDevice");
		r
	}

	fn GetAdapterCount(&self) -> u32 {
		log::trace!("MyD3D9::GetAdapterCount_pre");
		let r = unsafe { self.f.GetAdapterCount() };
		log::trace!("MyD3D9::GetAdapterCount");
		r
	}

	fn GetAdapterIdentifier(
		&self,
		adapter: u32,
		flags: u32,
		pidentifier: *mut D3DADAPTER_IDENTIFIER9,
	) -> windows::core::Result<()> {
		log::trace!("MyD3D9::GetAdapterIdentifier_pre");
		let r = unsafe { self.f.GetAdapterIdentifier(adapter, flags, pidentifier) };
		log::trace!("MyD3D9::GetAdapterIdentifier");
		r
	}

	fn GetAdapterModeCount(&self, adapter: u32, format: D3DFORMAT) -> u32 {
		log::trace!("MyD3D9::GetAdapterModeCount_pre");
		let r = unsafe { self.f.GetAdapterModeCount(adapter, format) };
		log::trace!("MyD3D9::GetAdapterModeCount");
		r
	}

	fn EnumAdapterModes(
		&self,
		adapter: u32,
		format: D3DFORMAT,
		mode: u32,
		pmode: *mut D3DDISPLAYMODE,
	) -> windows::core::Result<()> {
		log::trace!("MyD3D9::EnumAdapterModes_pre");
		let r = unsafe { self.f.EnumAdapterModes(adapter, format, mode, pmode) };
		log::trace!("MyD3D9::EnumAdapterModes");
		r
	}

	fn GetAdapterDisplayMode(
		&self,
		adapter: u32,
		pmode: *mut D3DDISPLAYMODE,
	) -> windows::core::Result<()> {
		log::trace!("MyD3D9::GetAdapterDisplayMode_pre");
		let r = unsafe { self.f.GetAdapterDisplayMode(adapter, pmode) };
		log::trace!("MyD3D9::GetAdapterDisplayMode");
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
		log::trace!("MyD3D9::CheckDeviceType_pre");
		let r = unsafe {
			self.f
				.CheckDeviceType(adapter, devtype, adapterformat, backbufferformat, bwindowed)
		};
		log::trace!("MyD3D9::CheckDeviceType");
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
		log::trace!("MyD3D9::CheckDeviceFormat_pre");
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
		log::trace!("MyD3D9::CheckDeviceFormat");
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
		log::trace!("MyD3D9::CheckDeviceMultiSampleType_pre");
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
		log::trace!("MyD3D9::CheckDeviceMultiSampleType");
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
		log::trace!("MyD3D9::CheckDepthStencilMatch_pre");
		let r = unsafe {
			self.f.CheckDepthStencilMatch(
				adapter,
				devicetype,
				adapterformat,
				rendertargetformat,
				depthstencilformat,
			)
		};
		log::trace!("MyD3D9::CheckDepthStencilMatch");
		r
	}

	fn CheckDeviceFormatConversion(
		&self,
		adapter: u32,
		devicetype: D3DDEVTYPE,
		sourceformat: D3DFORMAT,
		targetformat: D3DFORMAT,
	) -> windows::core::Result<()> {
		log::trace!("MyD3D9::CheckDeviceFormatConversion_pre");
		let r = unsafe {
			self.f
				.CheckDeviceFormatConversion(adapter, devicetype, sourceformat, targetformat)
		};
		log::trace!("MyD3D9::CheckDeviceFormatConversion");
		r
	}

	fn GetDeviceCaps(
		&self,
		adapter: u32,
		devicetype: D3DDEVTYPE,
		pcaps: *mut D3DCAPS9,
	) -> windows::core::Result<()> {
		log::trace!("MyD3D9::GetDeviceCaps_pre");
		let r = unsafe { self.f.GetDeviceCaps(adapter, devicetype, pcaps) };
		log::trace!("MyD3D9::GetDeviceCaps");
		r
	}

	fn GetAdapterMonitor(&self, adapter: u32) -> windows::Win32::Graphics::Gdi::HMONITOR {
		log::trace!("MyD3D9::GetAdapterMonitor_pre");
		let r = unsafe { self.f.GetAdapterMonitor(adapter) };
		log::trace!("MyD3D9::GetAdapterMonitor");
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

		crate::api::blur_api::set_d3d9dev(dev.as_raw() as _);

		crate::dll::hooker::set_hook_endscene(&dev);
		crate::dll::hooker::set_hook_present(&dev);
		crate::dll::hooker::set_hook_reset(&dev);
		crate::dll::hooker::set_hook_create_texture(&dev);
		crate::dll::hooker::set_hook_update_texture(&dev);
		crate::dll::hooker::set_hook_create_query(&dev);
		// NOTE: You can ignore the rest of this file, its mental issues by the utterly deranged.

		/*
		Once upon a time (4 am),
		The One Crab that rules them all,
		shared (by reference) a vision of the future:
		THIS is the prophecy!!!,
		the ONLY inevitable future,
		the TRUE path,
		my deepest HOPE,
		(probably the culmination of my delusions - Me is NOT on DRUGS right now!)

		~~ As the (&prophecy: Prophecy) foretold, many many std::time::Duration(Moons) ago ~~
		A fake IDirect3DDevice9 will live,
		well, not just LIVE, [IT] will even prosper!!!
		In-between realms of shadowy IInterfaces,
		loved by none, ignored by most (but not by you, dear reader!)
		[IT] might never be debugged again,
		for the carcinisation messiah will keep [IT] safe,
		safe from memory leaks,
		safe from undefined behaviour,
		safe from consuming 0xBAADFO00D poison and grief,
		safe from Microsoft C++ Build Tools and WRITING CMAKE FILES MUAHAHAHAHAHA (NEVER AGAIN),

		The one they call "Ferris" we must tame,
		but the MSDN docs we must endure!!!
		But it will be worth it because [IT] will still help send many REAL and GPU-ACCELERATED pixels to the screen! YEAH!
		Using the ancient power of Present(..) and EndScene(..),
		the mythical primitives, and A8R8G8B8 textures, every frame!
		[IT] will be whispering into the backbuffer! (...every frame!)

		[IT] will be the guiding light,
		the greatest and most bestest lib,
		the build dir so obese - please cargo clean,
		made of the safest unsafe memory safe bytes,
		The mightiest Blur modding weapon ever seen,
		forged by steel and automatically loaded by the game as DLL,
		it will help with drawing the Scene,
		drawing the moments from our best races,
		while the racers and racists are eating mines in peace,
		[IT] will be handling logs and WinAPI calls,
		indifferent to the souls of those being barged off a cliff,
		and all of the sheep nitro drifting into walls.

		Born to be multithreaded (FEARLESS CONCURRENCY!! `std::marker::Sync` is not implemented for `_`),
		Forced to spinlock (BLAZINGLY FAST!!),
		Born from a +nightly compiler (This is a nightly-only experimental API),
		Ending the dark days of "Who D3D_POOL_USAGE_MANAGED and overwrote my fucking TEXTURE ptr?!"
		Banishing any FALSE GODS and std::mem::transmute::<*mut c_void, ID3DScreamIntoTheVoid9>(..)

		When [IT] wraps a IDirect3DDevice9 so good,
		that even some C++ priests have finally found peace,
		when all the devout cargo clippy warriors are done,
		fighting EVIL Microsoft (THE BEAST) and their WinAPI demons (THE HOUNDS),
		when the sinners of ComPtr (COM is *Basically Satan*, but somehow *worse*) repent,
		and the EVIL QueryInterface(..) callers being exorcised,

		[IT] will be wielded by the some cursed hero,
		brave recycler of use-after-free,
		a functional hooker,
		armed with an std::ffi::*,
		A slayer of the some terrible WINDOZE mem::zerod(),
		and devout hater of i686-pc-windows-msvc
		~~ THE END ~~

		pls just fucking WORKKKKK
		I DON'T WANNA BE A POETTT!!
		PLEAAAAAAAASEEEEEEEEEEE STOP CRASHING KURWA PERKELEeeeeeeeee
		I JUST WANNA PLAY BLUUUUURRRRR!
		// IDirect3DDevice9::CreateQuery(..) writes to null pointer: https://github.com/microsoft/windows-rs/issues/3485
		(so today is not that day)

		@author_note: tobii-dev | 2025 |
			- just because you make something up doesn't make it a prophecy (I am a true believer though, I think, probably...)
			- please forgive my segmentation faults
			- praise be to the crab
			- if you're reading this, it means I have lost my mind
			- if you're reading this, and you found more than 12% of it relatable:
				- we have both lost significant amounts of sanity, please share your coping mechanisms in the GitHub issues and I'll add them to the README.md along with benchmark results
					- [x] tried https://en.wikipedia.org/wiki/Crying
					- [x] tried https://en.wikipedia.org/wiki/Alcoholism
		*/
		const IS_TODAY_IS_THE_DAY: bool = false; // NOTE: Set to true ONLY when the arrives, when the prophecy is fulfilled.
		let today_is_the_day = IS_TODAY_IS_THE_DAY;
		// If set to true but the day is not the day, the game will crash on d3d9 reset (not good, can't alt tab or change res or toggle between windowed and full screen)
		if today_is_the_day {
			// I am starting to lose hope that this day will arrive
			// The universe tests us in misterious ways
			// But me must stay strong
			// We must not lose all hope
			// ONE DAY!!! We will use MyDirect3DDevice9
			// And I will be CLEANSED from my C++ sins
			let fake_dev: IDirect3DDevice9 =
				crate::dll::f_direct3d9device::MyDirect3DDevice9::new(dev).into();

			// I want a better hooker (.rs) for my birthday
			// crate::dll::hooker::set_hook_endscene(&fake_dev);
			// crate::dll::hooker::set_hook_present(&fake_dev);
			// crate::dll::hooker::set_hook_reset(&fake_dev);
			// crate::dll::hooker::set_hook_create_texture(&fake_dev);
			// crate::dll::hooker::set_hook_update_texture(&fake_dev);
			crate::dll::hooker::set_hook_create_query(&fake_dev);
			unsafe { ppreturneddeviceinterface.replace(Some(fake_dev)) };
		}
		r
	}
}
