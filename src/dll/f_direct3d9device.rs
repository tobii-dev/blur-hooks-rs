#![allow(non_snake_case, non_camel_case_types)]

use windows::Win32::Graphics::Direct3D9::IDirect3D9;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9_Impl;

use windows::Win32::Graphics::Direct3D9::*;

use windows::core::implement;

#[derive(Debug)]
#[implement(IDirect3DDevice9)]
pub struct MyDirect3DDevice9 {
	f: IDirect3DDevice9,
}

impl MyDirect3DDevice9 {
	pub fn new(f: IDirect3DDevice9) -> Self {
		let r = MyDirect3DDevice9 { f };
		log::trace!("MyDirect3DDevice9::new() -> {r:#?}");
		r
	}
}

impl IDirect3DDevice9_Impl for MyDirect3DDevice9 {
	fn TestCooperativeLevel(&self) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::TestCooperativeLevel_pre");
		let r = unsafe { self.f.TestCooperativeLevel() };
		// log::info!("MyDirect3DDevice9::TestCooperativeLevel");
		r
	}

	fn GetAvailableTextureMem(&self) -> u32 {
		// log::info!("MyDirect3DDevice9::GetAvailableTextureMem_pre");
		let r = unsafe { self.f.GetAvailableTextureMem() };
		// log::info!("MyDirect3DDevice9::GetAvailableTextureMem");
		r
	}

	fn EvictManagedResources(&self) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::EvictManagedResources_pre");
		let r = unsafe { self.f.EvictManagedResources() };
		// log::info!("MyDirect3DDevice9::EvictManagedResources");
		r
	}

	fn GetDirect3D(&self) -> windows::core::Result<IDirect3D9> {
		log::info!("MyDirect3DDevice9::GetDirect3D_pre");
		let r = unsafe { self.f.GetDirect3D() };
		log::info!("MyDirect3DDevice9::GetDirect3D");
		r
	}

	fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetDeviceCaps_pre");
		let r = unsafe { self.f.GetDeviceCaps(pcaps) };
		// log::info!("MyDirect3DDevice9::GetDeviceCaps");
		r
	}

	fn GetDisplayMode(
		&self,
		iswapchain: u32,
		pmode: *mut D3DDISPLAYMODE,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetDisplayMode_pre");
		let r = unsafe { self.f.GetDisplayMode(iswapchain, pmode) };
		// log::info!("MyDirect3DDevice9::GetDisplayMode");
		r
	}

	fn GetCreationParameters(
		&self,
		pparameters: *mut D3DDEVICE_CREATION_PARAMETERS,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetCreationParameters_pre");
		let r = unsafe { self.f.GetCreationParameters(pparameters) };
		// log::info!("MyDirect3DDevice9::GetCreationParameters");
		r
	}

	fn SetCursorProperties(
		&self,
		xhotspot: u32,
		yhotspot: u32,
		pcursorbitmap: Option<&IDirect3DSurface9>,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetCursorProperties_pre");
		let r = unsafe {
			self.f
				.SetCursorProperties(xhotspot, yhotspot, pcursorbitmap)
		};
		// log::info!("MyDirect3DDevice9::SetCursorProperties");
		r
	}

	fn SetCursorPosition(&self, x: i32, y: i32, flags: u32) {
		// log::info!("MyDirect3DDevice9::SetCursorPosition_pre");
		unsafe { self.f.SetCursorPosition(x, y, flags) };
		// log::info!("MyDirect3DDevice9::SetCursorPosition");
	}

	fn ShowCursor(
		&self,
		bshow: windows::Win32::Foundation::BOOL,
	) -> windows::Win32::Foundation::BOOL {
		// log::info!("MyDirect3DDevice9::ShowCursor_pre");
		let r = unsafe { self.f.ShowCursor(bshow) };
		// log::info!("MyDirect3DDevice9::ShowCursor");
		r
	}

	fn CreateAdditionalSwapChain(
		&self,
		ppresentationparameters: *mut D3DPRESENT_PARAMETERS,
		pswapchain: *mut Option<IDirect3DSwapChain9>,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::CreateAdditionalSwapChain_pre");
		let r = unsafe {
			self.f
				.CreateAdditionalSwapChain(ppresentationparameters, pswapchain)
		};
		// log::info!("MyDirect3DDevice9::CreateAdditionalSwapChain");
		r
	}

	fn GetSwapChain(&self, iswapchain: u32) -> windows::core::Result<IDirect3DSwapChain9> {
		// log::info!("MyDirect3DDevice9::GetSwapChain_pre");
		let r = unsafe { self.f.GetSwapChain(iswapchain) };
		// log::info!("MyDirect3DDevice9::GetSwapChain");
		r
	}

	fn GetNumberOfSwapChains(&self) -> u32 {
		// log::info!("MyDirect3DDevice9::GetNumberOfSwapChains_pre");
		let r = unsafe { self.f.GetNumberOfSwapChains() };
		// log::info!("MyDirect3DDevice9::GetNumberOfSwapChains");
		r
	}

	fn Reset(
		&self,
		ppresentationparameters: *mut D3DPRESENT_PARAMETERS,
	) -> windows::core::Result<()> {
		log::info!("MyDirect3DDevice9::Reset_pre");
		let r = unsafe { self.f.Reset(ppresentationparameters) };
		log::info!("MyDirect3DDevice9::Reset");

		// dumb thing to help in debug
		let mut cont = false;
		let cont_ptr: &mut bool = &mut cont;
		while !*cont_ptr {
			*cont_ptr = false;
		}
		r
	}

	fn Present(
		&self,
		psourcerect: *const windows::Win32::Foundation::RECT,
		pdestrect: *const windows::Win32::Foundation::RECT,
		hdestwindowoverride: windows::Win32::Foundation::HWND,
		pdirtyregion: *const windows::Win32::Graphics::Gdi::RGNDATA,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::Present_pre");
		let r = unsafe {
			self.f
				.Present(psourcerect, pdestrect, hdestwindowoverride, pdirtyregion)
		};
		// log::info!("MyDirect3DDevice9::Present");
		r
	}

	fn GetBackBuffer(
		&self,
		iswapchain: u32,
		ibackbuffer: u32,
		backbuffer_type: D3DBACKBUFFER_TYPE,
	) -> windows::core::Result<IDirect3DSurface9> {
		// log::info!("MyDirect3DDevice9::GetBackBuffer_pre");
		let r = unsafe {
			self.f
				.GetBackBuffer(iswapchain, ibackbuffer, backbuffer_type)
		};
		// log::info!("MyDirect3DDevice9::GetBackBuffer");
		r
	}

	fn GetRasterStatus(
		&self,
		iswapchain: u32,
		prasterstatus: *mut D3DRASTER_STATUS,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetRasterStatus_pre");
		let r = unsafe { self.f.GetRasterStatus(iswapchain, prasterstatus) };
		// log::info!("MyDirect3DDevice9::GetRasterStatus");
		r
	}

	fn SetDialogBoxMode(
		&self,
		benabledialogs: windows::Win32::Foundation::BOOL,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetDialogBoxMode_pre");
		let r = unsafe { self.f.SetDialogBoxMode(benabledialogs) };
		// log::info!("MyDirect3DDevice9::SetDialogBoxMode");
		r
	}

	fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
		// log::info!("MyDirect3DDevice9::SetGammaRamp_pre");
		unsafe { self.f.SetGammaRamp(iswapchain, flags, pramp) };
		// log::info!("MyDirect3DDevice9::SetGammaRamp");
	}

	fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
		// log::info!("MyDirect3DDevice9::GetGammaRamp_pre");
		unsafe { self.f.GetGammaRamp(iswapchain, pramp) };
		// log::info!("MyDirect3DDevice9::GetGammaRamp");
	}

	fn CreateTexture(
		&self,
		width: u32,
		height: u32,
		levels: u32,
		usage: u32,
		format: D3DFORMAT,
		pool: D3DPOOL,
		pptexture: *mut Option<IDirect3DTexture9>,
		psharedhandle: *mut windows::Win32::Foundation::HANDLE,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::CreateTexture_pre");
		let r = unsafe {
			self.f.CreateTexture(
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
		// log::info!("MyDirect3DDevice9::CreateTexture");
		r
	}

	fn CreateVolumeTexture(
		&self,
		width: u32,
		height: u32,
		depth: u32,
		levels: u32,
		usage: u32,
		format: D3DFORMAT,
		pool: D3DPOOL,
		ppvolumetexture: *mut Option<IDirect3DVolumeTexture9>,
		psharedhandle: *mut windows::Win32::Foundation::HANDLE,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::CreateVolumeTexture_pre");
		let r = unsafe {
			self.f.CreateVolumeTexture(
				width,
				height,
				depth,
				levels,
				usage,
				format,
				pool,
				ppvolumetexture,
				psharedhandle,
			)
		};
		// log::info!("MyDirect3DDevice9::CreateVolumeTexture");
		r
	}

	fn CreateCubeTexture(
		&self,
		edgelength: u32,
		levels: u32,
		usage: u32,
		format: D3DFORMAT,
		pool: D3DPOOL,
		ppcubetexture: *mut Option<IDirect3DCubeTexture9>,
		psharedhandle: *mut windows::Win32::Foundation::HANDLE,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::CreateCubeTexture_pre");
		let r = unsafe {
			self.f.CreateCubeTexture(
				edgelength,
				levels,
				usage,
				format,
				pool,
				ppcubetexture,
				psharedhandle,
			)
		};
		// log::info!("MyDirect3DDevice9::CreateCubeTexture");
		r
	}

	fn CreateVertexBuffer(
		&self,
		length: u32,
		usage: u32,
		fvf: u32,
		pool: D3DPOOL,
		ppvertexbuffer: *mut Option<IDirect3DVertexBuffer9>,
		psharedhandle: *mut windows::Win32::Foundation::HANDLE,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::CreateVertexBuffer_pre");
		let r = unsafe {
			self.f
				.CreateVertexBuffer(length, usage, fvf, pool, ppvertexbuffer, psharedhandle)
		};
		// log::info!("MyDirect3DDevice9::CreateVertexBuffer");
		r
	}

	fn CreateIndexBuffer(
		&self,
		length: u32,
		usage: u32,
		format: D3DFORMAT,
		pool: D3DPOOL,
		ppindexbuffer: *mut Option<IDirect3DIndexBuffer9>,
		psharedhandle: *mut windows::Win32::Foundation::HANDLE,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::CreateIndexBuffer_pre");
		let r = unsafe {
			self.f
				.CreateIndexBuffer(length, usage, format, pool, ppindexbuffer, psharedhandle)
		};
		// log::info!("MyDirect3DDevice9::CreateIndexBuffer");
		r
	}

	fn CreateRenderTarget(
		&self,
		width: u32,
		height: u32,
		format: D3DFORMAT,
		multisample: D3DMULTISAMPLE_TYPE,
		multisamplequality: u32,
		lockable: windows::Win32::Foundation::BOOL,
		ppsurface: *mut Option<IDirect3DSurface9>,
		psharedhandle: *mut windows::Win32::Foundation::HANDLE,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::CreateRenderTarget_pre");
		let r = unsafe {
			self.f.CreateRenderTarget(
				width,
				height,
				format,
				multisample,
				multisamplequality,
				lockable,
				ppsurface,
				psharedhandle,
			)
		};
		// log::info!("MyDirect3DDevice9::CreateRenderTarget");
		r
	}

	fn CreateDepthStencilSurface(
		&self,
		width: u32,
		height: u32,
		format: D3DFORMAT,
		multisample: D3DMULTISAMPLE_TYPE,
		multisamplequality: u32,
		discard: windows::Win32::Foundation::BOOL,
		ppsurface: *mut Option<IDirect3DSurface9>,
		psharedhandle: *mut windows::Win32::Foundation::HANDLE,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::CreateDepthStencilSurface_pre");
		let r = unsafe {
			self.f.CreateDepthStencilSurface(
				width,
				height,
				format,
				multisample,
				multisamplequality,
				discard,
				ppsurface,
				psharedhandle,
			)
		};
		// log::info!("MyDirect3DDevice9::CreateDepthStencilSurface");
		r
	}

	fn UpdateSurface(
		&self,
		psourcesurface: Option<&IDirect3DSurface9>,
		psourcerect: *const windows::Win32::Foundation::RECT,
		pdestinationsurface: Option<&IDirect3DSurface9>,
		pdestpoint: *const windows::Win32::Foundation::POINT,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::UpdateSurface_pre");
		let r = unsafe {
			self.f
				.UpdateSurface(psourcesurface, psourcerect, pdestinationsurface, pdestpoint)
		};
		// log::info!("MyDirect3DDevice9::UpdateSurface");
		r
	}

	fn UpdateTexture(
		&self,
		psourcetexture: Option<&IDirect3DBaseTexture9>,
		pdestinationtexture: Option<&IDirect3DBaseTexture9>,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::UpdateTexture_pre");
		let r = unsafe { self.f.UpdateTexture(psourcetexture, pdestinationtexture) };
		// log::info!("MyDirect3DDevice9::UpdateTexture");
		r
	}

	fn GetRenderTargetData(
		&self,
		prendertarget: Option<&IDirect3DSurface9>,
		pdestsurface: Option<&IDirect3DSurface9>,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetRenderTargetData_pre");
		let r = unsafe { self.f.GetRenderTargetData(prendertarget, pdestsurface) };
		// log::info!("MyDirect3DDevice9::GetRenderTargetData");
		r
	}

	fn GetFrontBufferData(
		&self,
		iswapchain: u32,
		pdestsurface: Option<&IDirect3DSurface9>,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetFrontBufferData_pre");
		let r = unsafe { self.f.GetFrontBufferData(iswapchain, pdestsurface) };
		// log::info!("MyDirect3DDevice9::GetFrontBufferData");
		r
	}

	fn StretchRect(
		&self,
		psourcesurface: Option<&IDirect3DSurface9>,
		psourcerect: *const windows::Win32::Foundation::RECT,
		pdestsurface: Option<&IDirect3DSurface9>,
		pdestrect: *const windows::Win32::Foundation::RECT,
		filter: D3DTEXTUREFILTERTYPE,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::StretchRect_pre");
		let r = unsafe {
			self.f
				.StretchRect(psourcesurface, psourcerect, pdestsurface, pdestrect, filter)
		};
		// log::info!("MyDirect3DDevice9::StretchRect");
		r
	}

	fn ColorFill(
		&self,
		psurface: Option<&IDirect3DSurface9>,
		prect: *const windows::Win32::Foundation::RECT,
		color: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::ColorFill_pre");
		let r = unsafe { self.f.ColorFill(psurface, prect, color) };
		// log::info!("MyDirect3DDevice9::ColorFill");
		r
	}

	fn CreateOffscreenPlainSurface(
		&self,
		width: u32,
		height: u32,
		format: D3DFORMAT,
		pool: D3DPOOL,
		ppsurface: *mut Option<IDirect3DSurface9>,
		psharedhandle: *mut windows::Win32::Foundation::HANDLE,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::CreateOffscreenPlainSurface_pre");
		let r = unsafe {
			self.f.CreateOffscreenPlainSurface(
				width,
				height,
				format,
				pool,
				ppsurface,
				psharedhandle,
			)
		};
		// log::info!("MyDirect3DDevice9::CreateOffscreenPlainSurface");
		r
	}

	fn SetRenderTarget(
		&self,
		rendertargetindex: u32,
		prendertarget: Option<&IDirect3DSurface9>,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetRenderTarget_pre");
		let r = unsafe { self.f.SetRenderTarget(rendertargetindex, prendertarget) };
		// log::info!("MyDirect3DDevice9::SetRenderTarget");
		r
	}

	fn GetRenderTarget(&self, rendertargetindex: u32) -> windows::core::Result<IDirect3DSurface9> {
		// log::info!("MyDirect3DDevice9::GetRenderTarget_pre");
		let r = unsafe { self.f.GetRenderTarget(rendertargetindex) };
		// log::info!("MyDirect3DDevice9::GetRenderTarget");
		r
	}

	fn SetDepthStencilSurface(
		&self,
		pnewzstencil: Option<&IDirect3DSurface9>,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetDepthStencilSurface_pre");
		let r = unsafe { self.f.SetDepthStencilSurface(pnewzstencil) };
		// log::info!("MyDirect3DDevice9::SetDepthStencilSurface");
		r
	}

	fn GetDepthStencilSurface(&self) -> windows::core::Result<IDirect3DSurface9> {
		// log::info!("MyDirect3DDevice9::GetDepthStencilSurface_pre");
		let r = unsafe { self.f.GetDepthStencilSurface() };
		// log::info!("MyDirect3DDevice9::GetDepthStencilSurface");
		r
	}

	fn BeginScene(&self) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::BeginScene_pre");
		let r = unsafe { self.f.BeginScene() };
		// log::info!("MyDirect3DDevice9::BeginScene");
		r
	}

	fn EndScene(&self) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::EndScene_pre");
		let r = unsafe { self.f.EndScene() };
		// log::info!("MyDirect3DDevice9::EndScene");
		r
	}

	fn Clear(
		&self,
		count: u32,
		prects: *const D3DRECT,
		flags: u32,
		color: u32,
		z: f32,
		stencil: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::Clear_pre");
		let r = unsafe { self.f.Clear(count, prects, flags, color, z, stencil) };
		// log::info!("MyDirect3DDevice9::Clear");
		r
	}

	fn SetTransform(
		&self,
		state: D3DTRANSFORMSTATETYPE,
		pmatrix: *const windows::Foundation::Numerics::Matrix4x4,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetTransform_pre");
		let r = unsafe { self.f.SetTransform(state, pmatrix) };
		// log::info!("MyDirect3DDevice9::SetTransform");
		r
	}

	fn GetTransform(
		&self,
		state: D3DTRANSFORMSTATETYPE,
		pmatrix: *mut windows::Foundation::Numerics::Matrix4x4,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetTransform_pre");
		let r = unsafe { self.f.GetTransform(state, pmatrix) };
		// log::info!("MyDirect3DDevice9::GetTransform");
		r
	}

	fn MultiplyTransform(
		&self,
		param0: D3DTRANSFORMSTATETYPE,
		param1: *const windows::Foundation::Numerics::Matrix4x4,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::MultiplyTransform_pre");
		let r = unsafe { self.f.MultiplyTransform(param0, param1) };
		// log::info!("MyDirect3DDevice9::MultiplyTransform");
		r
	}

	fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetViewport_pre");
		let r = unsafe { self.f.SetViewport(pviewport) };
		// log::info!("MyDirect3DDevice9::SetViewport");
		r
	}

	fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetViewport_pre");
		let r = unsafe { self.f.GetViewport(pviewport) };
		// log::info!("MyDirect3DDevice9::GetViewport");
		r
	}

	fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetMaterial_pre");
		let r = unsafe { self.f.SetMaterial(pmaterial) };
		// log::info!("MyDirect3DDevice9::SetMaterial");
		r
	}

	fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetMaterial_pre");
		let r = unsafe { self.f.GetMaterial(pmaterial) };
		// log::info!("MyDirect3DDevice9::GetMaterial");
		r
	}

	fn SetLight(&self, index: u32, param1: *const D3DLIGHT9) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetLight_pre");
		let r = unsafe { self.f.SetLight(index, param1) };
		// log::info!("MyDirect3DDevice9::SetLight");
		r
	}

	fn GetLight(&self, index: u32, param1: *mut D3DLIGHT9) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetLight_pre");
		let r = unsafe { self.f.GetLight(index, param1) };
		// log::info!("MyDirect3DDevice9::GetLight");
		r
	}

	fn LightEnable(
		&self,
		index: u32,
		enable: windows::Win32::Foundation::BOOL,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::LightEnable_pre");
		let r = unsafe { self.f.LightEnable(index, enable) };
		// log::info!("MyDirect3DDevice9::LightEnable");
		r
	}

	fn GetLightEnable(
		&self,
		index: u32,
		penable: *mut windows::Win32::Foundation::BOOL,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetLightEnable_pre");
		let r = unsafe { self.f.GetLightEnable(index, penable) };
		// log::info!("MyDirect3DDevice9::GetLightEnable");
		r
	}

	fn SetClipPlane(&self, index: u32, pplane: *const f32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetClipPlane_pre");
		let r = unsafe { self.f.SetClipPlane(index, pplane) };
		// log::info!("MyDirect3DDevice9::SetClipPlane");
		r
	}

	fn GetClipPlane(&self, index: u32, pplane: *mut f32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetClipPlane_pre");
		let r = unsafe { self.f.GetClipPlane(index, pplane) };
		// log::info!("MyDirect3DDevice9::GetClipPlane");
		r
	}

	fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: u32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetRenderState_pre");
		let r = unsafe { self.f.SetRenderState(state, value) };
		// log::info!("MyDirect3DDevice9::SetRenderState");
		r
	}

	fn GetRenderState(
		&self,
		state: D3DRENDERSTATETYPE,
		pvalue: *mut u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetRenderState_pre");
		let r = unsafe { self.f.GetRenderState(state, pvalue) };
		// log::info!("MyDirect3DDevice9::GetRenderState");
		r
	}

	fn CreateStateBlock(
		&self,
		stateblock_type: D3DSTATEBLOCKTYPE,
	) -> windows::core::Result<IDirect3DStateBlock9> {
		// log::info!("MyDirect3DDevice9::CreateStateBlock_pre");
		let r = unsafe { self.f.CreateStateBlock(stateblock_type) };
		// log::info!("MyDirect3DDevice9::CreateStateBlock");
		r
	}

	fn BeginStateBlock(&self) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::BeginStateBlock_pre");
		let r = unsafe { self.f.BeginStateBlock() };
		// log::info!("MyDirect3DDevice9::BeginStateBlock");
		r
	}

	fn EndStateBlock(&self) -> windows::core::Result<IDirect3DStateBlock9> {
		// log::info!("MyDirect3DDevice9::EndStateBlock_pre");
		let r = unsafe { self.f.EndStateBlock() };
		// log::info!("MyDirect3DDevice9::EndStateBlock");
		r
	}

	fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetClipStatus_pre");
		let r = unsafe { self.f.SetClipStatus(pclipstatus) };
		// log::info!("MyDirect3DDevice9::SetClipStatus");
		r
	}

	fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetClipStatus_pre");
		let r = unsafe { self.f.GetClipStatus(pclipstatus) };
		// log::info!("MyDirect3DDevice9::GetClipStatus");
		r
	}

	fn GetTexture(&self, stage: u32) -> windows::core::Result<IDirect3DBaseTexture9> {
		// log::info!("MyDirect3DDevice9::GetTexture_pre");
		let r = unsafe { self.f.GetTexture(stage) };
		// log::info!("MyDirect3DDevice9::GetTexture");
		r
	}

	fn SetTexture(
		&self,
		stage: u32,
		ptexture: Option<&IDirect3DBaseTexture9>,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetTexture_pre");
		let r = unsafe { self.f.SetTexture(stage, ptexture) };
		// log::info!("MyDirect3DDevice9::SetTexture");
		r
	}

	fn GetTextureStageState(
		&self,
		stage: u32,
		texstage_type: D3DTEXTURESTAGESTATETYPE,
		pvalue: *mut u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetTextureStageState_pre");
		let r = unsafe { self.f.GetTextureStageState(stage, texstage_type, pvalue) };
		// log::info!("MyDirect3DDevice9::GetTextureStageState");
		r
	}

	fn SetTextureStageState(
		&self,
		stage: u32,
		texstage_type: D3DTEXTURESTAGESTATETYPE,
		value: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetTextureStageState_pre");
		let r = unsafe { self.f.SetTextureStageState(stage, texstage_type, value) };
		// log::info!("MyDirect3DDevice9::SetTextureStageState");
		r
	}

	fn GetSamplerState(
		&self,
		sampler: u32,
		samplerstate_type: D3DSAMPLERSTATETYPE,
		pvalue: *mut u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetSamplerState_pre");
		let r = unsafe { self.f.GetSamplerState(sampler, samplerstate_type, pvalue) };
		// log::info!("MyDirect3DDevice9::GetSamplerState");
		r
	}

	fn SetSamplerState(
		&self,
		sampler: u32,
		samplerstate_type: D3DSAMPLERSTATETYPE,
		value: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetSamplerState_pre");
		let r = unsafe { self.f.SetSamplerState(sampler, samplerstate_type, value) };
		// log::info!("MyDirect3DDevice9::SetSamplerState");
		r
	}

	fn ValidateDevice(&self, pnumpasses: *mut u32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::ValidateDevice_pre");
		let r = unsafe { self.f.ValidateDevice(pnumpasses) };
		// log::info!("MyDirect3DDevice9::ValidateDevice");
		r
	}

	fn SetPaletteEntries(
		&self,
		palettenumber: u32,
		pentries: *const windows::Win32::Graphics::Gdi::PALETTEENTRY,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetPaletteEntries_pre");
		let r = unsafe { self.f.SetPaletteEntries(palettenumber, pentries) };
		// log::info!("MyDirect3DDevice9::SetPaletteEntries");
		r
	}

	fn GetPaletteEntries(
		&self,
		palettenumber: u32,
		pentries: *mut windows::Win32::Graphics::Gdi::PALETTEENTRY,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetPaletteEntries_pre");
		let r = unsafe { self.f.GetPaletteEntries(palettenumber, pentries) };
		// log::info!("MyDirect3DDevice9::GetPaletteEntries");
		r
	}

	fn SetCurrentTexturePalette(&self, palettenumber: u32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetCurrentTexturePalette_pre");
		let r = unsafe { self.f.SetCurrentTexturePalette(palettenumber) };
		// log::info!("MyDirect3DDevice9::SetCurrentTexturePalette");
		r
	}

	fn GetCurrentTexturePalette(&self, palettenumber: *mut u32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetCurrentTexturePalette_pre");
		let r = unsafe { self.f.GetCurrentTexturePalette(palettenumber) };
		// log::info!("MyDirect3DDevice9::GetCurrentTexturePalette");
		r
	}

	fn SetScissorRect(
		&self,
		prect: *const windows::Win32::Foundation::RECT,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetScissorRect_pre");
		let r = unsafe { self.f.SetScissorRect(prect) };
		// log::info!("MyDirect3DDevice9::SetScissorRect");
		r
	}

	fn GetScissorRect(
		&self,
		prect: *mut windows::Win32::Foundation::RECT,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetScissorRect_pre");
		let r = unsafe { self.f.GetScissorRect(prect) };
		// log::info!("MyDirect3DDevice9::GetScissorRect");
		r
	}

	fn SetSoftwareVertexProcessing(
		&self,
		bsoftware: windows::Win32::Foundation::BOOL,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetSoftwareVertexProcessing_pre");
		let r = unsafe { self.f.SetSoftwareVertexProcessing(bsoftware) };
		// log::info!("MyDirect3DDevice9::SetSoftwareVertexProcessing");
		r
	}

	fn GetSoftwareVertexProcessing(&self) -> windows::Win32::Foundation::BOOL {
		// log::info!("MyDirect3DDevice9::GetSoftwareVertexProcessing_pre");
		let r = unsafe { self.f.GetSoftwareVertexProcessing() };
		// log::info!("MyDirect3DDevice9::GetSoftwareVertexProcessing");
		r
	}

	fn SetNPatchMode(&self, nsegments: f32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetNPatchMode_pre");
		let r = unsafe { self.f.SetNPatchMode(nsegments) };
		// log::info!("MyDirect3DDevice9::SetNPatchMode");
		r
	}

	fn GetNPatchMode(&self) -> f32 {
		// log::info!("MyDirect3DDevice9::GetNPatchMode_pre");
		let r = unsafe { self.f.GetNPatchMode() };
		// log::info!("MyDirect3DDevice9::GetNPatchMode");
		r
	}

	fn DrawPrimitive(
		&self,
		primitivetype: D3DPRIMITIVETYPE,
		startvertex: u32,
		primitivecount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::DrawPrimitive_pre");
		let r = unsafe {
			self.f
				.DrawPrimitive(primitivetype, startvertex, primitivecount)
		};
		// log::info!("MyDirect3DDevice9::DrawPrimitive");
		r
	}

	fn DrawIndexedPrimitive(
		&self,
		param0: D3DPRIMITIVETYPE,
		basevertexindex: i32,
		minvertexindex: u32,
		numvertices: u32,
		startindex: u32,
		primcount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::DrawIndexedPrimitive_pre");
		let r = unsafe {
			self.f.DrawIndexedPrimitive(
				param0,
				basevertexindex,
				minvertexindex,
				numvertices,
				startindex,
				primcount,
			)
		};
		// log::info!("MyDirect3DDevice9::DrawIndexedPrimitive");
		r
	}

	fn DrawPrimitiveUP(
		&self,
		primitivetype: D3DPRIMITIVETYPE,
		primitivecount: u32,
		pvertexstreamzerodata: *const core::ffi::c_void,
		vertexstreamzerostride: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::DrawPrimitiveUP_pre");
		let r = unsafe {
			self.f.DrawPrimitiveUP(
				primitivetype,
				primitivecount,
				pvertexstreamzerodata,
				vertexstreamzerostride,
			)
		};
		// log::info!("MyDirect3DDevice9::DrawPrimitiveUP");
		r
	}

	fn DrawIndexedPrimitiveUP(
		&self,
		primitivetype: D3DPRIMITIVETYPE,
		minvertexindex: u32,
		numvertices: u32,
		primitivecount: u32,
		pindexdata: *const core::ffi::c_void,
		indexdataformat: D3DFORMAT,
		pvertexstreamzerodata: *const core::ffi::c_void,
		vertexstreamzerostride: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::DrawIndexedPrimitiveUP_pre");
		let r = unsafe {
			self.f.DrawIndexedPrimitiveUP(
				primitivetype,
				minvertexindex,
				numvertices,
				primitivecount,
				pindexdata,
				indexdataformat,
				pvertexstreamzerodata,
				vertexstreamzerostride,
			)
		};
		// log::info!("MyDirect3DDevice9::DrawIndexedPrimitiveUP");
		r
	}

	fn ProcessVertices(
		&self,
		srcstartindex: u32,
		destindex: u32,
		vertexcount: u32,
		pdestbuffer: Option<&IDirect3DVertexBuffer9>,
		pvertexdecl: Option<&IDirect3DVertexDeclaration9>,
		flags: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::ProcessVertices_pre");
		let r = unsafe {
			self.f.ProcessVertices(
				srcstartindex,
				destindex,
				vertexcount,
				pdestbuffer,
				pvertexdecl,
				flags,
			)
		};
		// log::info!("MyDirect3DDevice9::ProcessVertices");
		r
	}

	fn CreateVertexDeclaration(
		&self,
		pvertexelements: *const D3DVERTEXELEMENT9,
	) -> windows::core::Result<IDirect3DVertexDeclaration9> {
		// log::info!("MyDirect3DDevice9::CreateVertexDeclaration_pre");
		let r = unsafe { self.f.CreateVertexDeclaration(pvertexelements) };
		// log::info!("MyDirect3DDevice9::CreateVertexDeclaration");
		r
	}

	fn SetVertexDeclaration(
		&self,
		pdecl: Option<&IDirect3DVertexDeclaration9>,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetVertexDeclaration_pre");
		let r = unsafe { self.f.SetVertexDeclaration(pdecl) };
		// log::info!("MyDirect3DDevice9::SetVertexDeclaration");
		r
	}

	fn GetVertexDeclaration(&self) -> windows::core::Result<IDirect3DVertexDeclaration9> {
		// log::info!("MyDirect3DDevice9::GetVertexDeclaration_pre");
		let r = unsafe { self.f.GetVertexDeclaration() };
		// log::info!("MyDirect3DDevice9::GetVertexDeclaration");
		r
	}

	fn SetFVF(&self, fvf: u32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetFVF_pre");
		let r = unsafe { self.f.SetFVF(fvf) };
		// log::info!("MyDirect3DDevice9::SetFVF");
		r
	}

	fn GetFVF(&self, pfvf: *mut u32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetFVF_pre");
		let r = unsafe { self.f.GetFVF(pfvf) };
		// log::info!("MyDirect3DDevice9::GetFVF");
		r
	}

	fn CreateVertexShader(
		&self,
		pfunction: *const u32,
	) -> windows::core::Result<IDirect3DVertexShader9> {
		// log::info!("MyDirect3DDevice9::CreateVertexShader_pre");
		let r = unsafe { self.f.CreateVertexShader(pfunction) };
		// log::info!("MyDirect3DDevice9::CreateVertexShader");
		r
	}

	fn SetVertexShader(
		&self,
		pshader: Option<&IDirect3DVertexShader9>,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetVertexShader_pre");
		let r = unsafe { self.f.SetVertexShader(pshader) };
		// log::info!("MyDirect3DDevice9::SetVertexShader");
		r
	}

	fn GetVertexShader(&self) -> windows::core::Result<IDirect3DVertexShader9> {
		// log::info!("MyDirect3DDevice9::GetVertexShader_pre");
		let r = unsafe { self.f.GetVertexShader() };
		// log::info!("MyDirect3DDevice9::GetVertexShader");
		r
	}

	fn SetVertexShaderConstantF(
		&self,
		startregister: u32,
		pconstantdata: *const f32,
		vector4fcount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetVertexShaderConstantF_pre");
		let r = unsafe {
			self.f
				.SetVertexShaderConstantF(startregister, pconstantdata, vector4fcount)
		};
		// log::info!("MyDirect3DDevice9::SetVertexShaderConstantF");
		r
	}

	fn GetVertexShaderConstantF(
		&self,
		startregister: u32,
		pconstantdata: *mut f32,
		vector4fcount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetVertexShaderConstantF_pre");
		let r = unsafe {
			self.f
				.GetVertexShaderConstantF(startregister, pconstantdata, vector4fcount)
		};
		// log::info!("MyDirect3DDevice9::GetVertexShaderConstantF");
		r
	}

	fn SetVertexShaderConstantI(
		&self,
		startregister: u32,
		pconstantdata: *const i32,
		vector4icount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetVertexShaderConstantI_pre");
		let r = unsafe {
			self.f
				.SetVertexShaderConstantI(startregister, pconstantdata, vector4icount)
		};
		// log::info!("MyDirect3DDevice9::SetVertexShaderConstantI");
		r
	}

	fn GetVertexShaderConstantI(
		&self,
		startregister: u32,
		pconstantdata: *mut i32,
		vector4icount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetVertexShaderConstantI_pre");
		let r = unsafe {
			self.f
				.GetVertexShaderConstantI(startregister, pconstantdata, vector4icount)
		};
		// log::info!("MyDirect3DDevice9::GetVertexShaderConstantI");
		r
	}

	fn SetVertexShaderConstantB(
		&self,
		startregister: u32,
		pconstantdata: *const windows::Win32::Foundation::BOOL,
		boolcount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetVertexShaderConstantB_pre");
		let r = unsafe {
			self.f
				.SetVertexShaderConstantB(startregister, pconstantdata, boolcount)
		};
		// log::info!("MyDirect3DDevice9::SetVertexShaderConstantB");
		r
	}

	fn GetVertexShaderConstantB(
		&self,
		startregister: u32,
		pconstantdata: *mut windows::Win32::Foundation::BOOL,
		boolcount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetVertexShaderConstantB_pre");
		let r = unsafe {
			self.f
				.GetVertexShaderConstantB(startregister, pconstantdata, boolcount)
		};
		// log::info!("MyDirect3DDevice9::GetVertexShaderConstantB");
		r
	}

	fn SetStreamSource(
		&self,
		streamnumber: u32,
		pstreamdata: Option<&IDirect3DVertexBuffer9>,
		offsetinbytes: u32,
		stride: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetStreamSource_pre");
		let r = unsafe {
			self.f
				.SetStreamSource(streamnumber, pstreamdata, offsetinbytes, stride)
		};
		// log::info!("MyDirect3DDevice9::SetStreamSource");
		r
	}

	fn GetStreamSource(
		&self,
		streamnumber: u32,
		ppstreamdata: *mut Option<IDirect3DVertexBuffer9>,
		poffsetinbytes: *mut u32,
		pstride: *mut u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetStreamSource_pre");
		let r = unsafe {
			self.f
				.GetStreamSource(streamnumber, ppstreamdata, poffsetinbytes, pstride)
		};
		// log::info!("MyDirect3DDevice9::GetStreamSource");
		r
	}

	fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetStreamSourceFreq_pre");
		let r = unsafe { self.f.SetStreamSourceFreq(streamnumber, setting) };
		// log::info!("MyDirect3DDevice9::SetStreamSourceFreq");
		r
	}

	fn GetStreamSourceFreq(
		&self,
		streamnumber: u32,
		psetting: *mut u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetStreamSourceFreq_pre");
		let r = unsafe { self.f.GetStreamSourceFreq(streamnumber, psetting) };
		// log::info!("MyDirect3DDevice9::GetStreamSourceFreq");
		r
	}

	fn SetIndices(&self, pindexdata: Option<&IDirect3DIndexBuffer9>) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetIndices_pre");
		let r = unsafe { self.f.SetIndices(pindexdata) };
		// log::info!("MyDirect3DDevice9::SetIndices");
		r
	}

	fn GetIndices(&self) -> windows::core::Result<IDirect3DIndexBuffer9> {
		// log::info!("MyDirect3DDevice9::GetIndices_pre");
		let r = unsafe { self.f.GetIndices() };
		// log::info!("MyDirect3DDevice9::GetIndices");
		r
	}

	fn CreatePixelShader(
		&self,
		pfunction: *const u32,
	) -> windows::core::Result<IDirect3DPixelShader9> {
		// log::info!("MyDirect3DDevice9::CreatePixelShader_pre");
		let r = unsafe { self.f.CreatePixelShader(pfunction) };
		// log::info!("MyDirect3DDevice9::CreatePixelShader");
		r
	}

	fn SetPixelShader(&self, pshader: Option<&IDirect3DPixelShader9>) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetPixelShader_pre");
		let r = unsafe { self.f.SetPixelShader(pshader) };
		// log::info!("MyDirect3DDevice9::SetPixelShader");
		r
	}

	fn GetPixelShader(&self) -> windows::core::Result<IDirect3DPixelShader9> {
		// log::info!("MyDirect3DDevice9::GetPixelShader_pre");
		let r = unsafe { self.f.GetPixelShader() };
		// log::info!("MyDirect3DDevice9::GetPixelShader");
		r
	}

	fn SetPixelShaderConstantF(
		&self,
		startregister: u32,
		pconstantdata: *const f32,
		vector4fcount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetPixelShaderConstantF_pre");
		let r = unsafe {
			self.f
				.SetPixelShaderConstantF(startregister, pconstantdata, vector4fcount)
		};
		// log::info!("MyDirect3DDevice9::SetPixelShaderConstantF");
		r
	}

	fn GetPixelShaderConstantF(
		&self,
		startregister: u32,
		pconstantdata: *mut f32,
		vector4fcount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetPixelShaderConstantF_pre");
		let r = unsafe {
			self.f
				.GetPixelShaderConstantF(startregister, pconstantdata, vector4fcount)
		};
		// log::info!("MyDirect3DDevice9::GetPixelShaderConstantF");
		r
	}

	fn SetPixelShaderConstantI(
		&self,
		startregister: u32,
		pconstantdata: *const i32,
		vector4icount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetPixelShaderConstantI_pre");
		let r = unsafe {
			self.f
				.SetPixelShaderConstantI(startregister, pconstantdata, vector4icount)
		};
		// log::info!("MyDirect3DDevice9::SetPixelShaderConstantI");
		r
	}

	fn GetPixelShaderConstantI(
		&self,
		startregister: u32,
		pconstantdata: *mut i32,
		vector4icount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetPixelShaderConstantI_pre");
		let r = unsafe {
			self.f
				.GetPixelShaderConstantI(startregister, pconstantdata, vector4icount)
		};
		// log::info!("MyDirect3DDevice9::GetPixelShaderConstantI");
		r
	}

	fn SetPixelShaderConstantB(
		&self,
		startregister: u32,
		pconstantdata: *const windows::Win32::Foundation::BOOL,
		boolcount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::SetVertexShaderConstantB_pre");
		let r = unsafe {
			self.f
				.SetPixelShaderConstantB(startregister, pconstantdata, boolcount)
		};
		// log::info!("MyDirect3DDevice9::SetVertexShaderConstantB");
		r
	}

	fn GetPixelShaderConstantB(
		&self,
		startregister: u32,
		pconstantdata: *mut windows::Win32::Foundation::BOOL,
		boolcount: u32,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::GetPixelShaderConstantB_pre");
		let r = unsafe {
			self.f
				.GetPixelShaderConstantB(startregister, pconstantdata, boolcount)
		};
		// log::info!("MyDirect3DDevice9::GetPixelShaderConstantB");
		r
	}

	fn DrawRectPatch(
		&self,
		handle: u32,
		pnumsegs: *const f32,
		prectpatchinfo: *const D3DRECTPATCH_INFO,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::DrawRectPatch_pre");
		let r = unsafe { self.f.DrawRectPatch(handle, pnumsegs, prectpatchinfo) };
		// log::info!("MyDirect3DDevice9::DrawRectPatch");
		r
	}

	fn DrawTriPatch(
		&self,
		handle: u32,
		pnumsegs: *const f32,
		ptripatchinfo: *const D3DTRIPATCH_INFO,
	) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::DrawTriPatch_pre");
		let r = unsafe { self.f.DrawTriPatch(handle, pnumsegs, ptripatchinfo) };
		// log::info!("MyDirect3DDevice9::DrawTriPatch");
		r
	}

	fn DeletePatch(&self, handle: u32) -> windows::core::Result<()> {
		// log::info!("MyDirect3DDevice9::DeletePatch_pre");
		let r = unsafe { self.f.DeletePatch(handle) };
		// log::info!("MyDirect3DDevice9::DeletePatch");
		r
	}

	//FIXME: https://github.com/microsoft/windows-rs/issues/3485
	fn CreateQuery(&self, q_type: D3DQUERYTYPE) -> windows::core::Result<IDirect3DQuery9> {
		// log::info!("MyDirect3DDevice9::CreateQuery_pre(q_type = {q_type:?})");
		let r = unsafe { self.f.CreateQuery(q_type) };
		// log::info!("MyDirect3DDevice9::CreateQuery(q_type = {q_type:?}) -> {r:?}");
		r
	}
}
