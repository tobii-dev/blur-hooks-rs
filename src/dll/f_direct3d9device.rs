#![allow(non_snake_case, non_camel_case_types)]
use windows::Win32::Graphics::Direct3D9::IDirect3D9;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9_Impl;

use windows::Win32::Graphics::Direct3D9::*;

//use windows::core::Interface;
use windows::core::implement;

use log::info;

#[derive(Debug)]
#[implement(IDirect3DDevice9)]
pub struct MyDirect3DDevice9 {
    f: IDirect3DDevice9,
}

impl MyDirect3DDevice9 {
    pub fn new(f: IDirect3DDevice9) -> Self {
        let r = MyDirect3DDevice9 { f };
        info!("MyDirect3DDevice9::new() -> {r:#?}");
        r
    }
}

impl IDirect3DDevice9_Impl for MyDirect3DDevice9 {
    fn TestCooperativeLevel(&self) -> windows::core::Result<()> {
        info!("TestCooperativeLevel_pre");
        let r = unsafe { self.f.TestCooperativeLevel() };
        info!("TestCooperativeLevel");
        r
    }

    fn GetAvailableTextureMem(&self) -> u32 {
        info!("GetAvailableTextureMem_pre");
        let r = unsafe { self.f.GetAvailableTextureMem() };
        info!("GetAvailableTextureMem");
        r
    }

    fn EvictManagedResources(&self) -> windows::core::Result<()> {
        info!("EvictManagedResources_pre");
        let r = unsafe { self.f.EvictManagedResources() };
        info!("EvictManagedResources");
        r
    }

    fn GetDirect3D(&self) -> windows::core::Result<IDirect3D9> {
        info!("GetDirect3D_pre");
        let r = unsafe { self.f.GetDirect3D() };
        info!("GetDirect3D");
        r
    }

    fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> windows::core::Result<()> {
        info!("GetDeviceCaps_pre");
        let r = unsafe { self.f.GetDeviceCaps(pcaps) };
        info!("GetDeviceCaps");
        r
    }

    fn GetDisplayMode(
        &self,
        iswapchain: u32,
        pmode: *mut D3DDISPLAYMODE,
    ) -> windows::core::Result<()> {
        info!("GetDisplayMode_pre");
        let r = unsafe { self.f.GetDisplayMode(iswapchain, pmode) };
        info!("GetDisplayMode");
        r
    }

    fn GetCreationParameters(
        &self,
        pparameters: *mut D3DDEVICE_CREATION_PARAMETERS,
    ) -> windows::core::Result<()> {
        info!("GetCreationParameters_pre");
        let r = unsafe { self.f.GetCreationParameters(pparameters) };
        info!("GetCreationParameters");
        r
    }

    fn SetCursorProperties(
        &self,
        xhotspot: u32,
        yhotspot: u32,
        pcursorbitmap: Option<&IDirect3DSurface9>,
    ) -> windows::core::Result<()> {
        info!("SetCursorProperties_pre");
        let r = unsafe {
            self.f
                .SetCursorProperties(xhotspot, yhotspot, pcursorbitmap)
        };
        info!("SetCursorProperties");
        r
    }

    fn SetCursorPosition(&self, x: i32, y: i32, flags: u32) {
        info!("SetCursorPosition_pre");
        let r = unsafe { self.f.SetCursorPosition(x, y, flags) };
        info!("SetCursorPosition");
        r
    }

    fn ShowCursor(
        &self,
        bshow: windows::Win32::Foundation::BOOL,
    ) -> windows::Win32::Foundation::BOOL {
        info!("ShowCursor_pre");
        let r = unsafe { self.f.ShowCursor(bshow) };
        info!("ShowCursor");
        r
    }

    fn CreateAdditionalSwapChain(
        &self,
        ppresentationparameters: *mut D3DPRESENT_PARAMETERS,
        pswapchain: *mut Option<IDirect3DSwapChain9>,
    ) -> windows::core::Result<()> {
        info!("CreateAdditionalSwapChain_pre");
        let r = unsafe {
            self.f
                .CreateAdditionalSwapChain(ppresentationparameters, pswapchain)
        };
        info!("CreateAdditionalSwapChain");
        r
    }

    fn GetSwapChain(&self, iswapchain: u32) -> windows::core::Result<IDirect3DSwapChain9> {
        info!("GetSwapChain_pre");
        let r = unsafe { self.f.GetSwapChain(iswapchain) };
        info!("GetSwapChain");
        r
    }

    fn GetNumberOfSwapChains(&self) -> u32 {
        info!("GetNumberOfSwapChains_pre");
        let r = unsafe { self.f.GetNumberOfSwapChains() };
        info!("GetNumberOfSwapChains");
        r
    }

    fn Reset(
        &self,
        ppresentationparameters: *mut D3DPRESENT_PARAMETERS,
    ) -> windows::core::Result<()> {
        info!("Reset_pre");
        let r = unsafe { self.f.Reset(ppresentationparameters) };
        info!("Reset");
        r
    }

    fn Present(
        &self,
        psourcerect: *const windows::Win32::Foundation::RECT,
        pdestrect: *const windows::Win32::Foundation::RECT,
        hdestwindowoverride: windows::Win32::Foundation::HWND,
        pdirtyregion: *const windows::Win32::Graphics::Gdi::RGNDATA,
    ) -> windows::core::Result<()> {
        info!("Present_pre");
        let r = unsafe {
            self.f
                .Present(psourcerect, pdestrect, hdestwindowoverride, pdirtyregion)
        };
        info!("Present");
        r
    }

    fn GetBackBuffer(
        &self,
        iswapchain: u32,
        ibackbuffer: u32,
        backbuffer_type: D3DBACKBUFFER_TYPE,
    ) -> windows::core::Result<IDirect3DSurface9> {
        info!("GetBackBuffer_pre");
        let r = unsafe {
            self.f
                .GetBackBuffer(iswapchain, ibackbuffer, backbuffer_type)
        };
        info!("GetBackBuffer");
        r
    }

    fn GetRasterStatus(
        &self,
        iswapchain: u32,
        prasterstatus: *mut D3DRASTER_STATUS,
    ) -> windows::core::Result<()> {
        info!("GetRasterStatus_pre");
        let r = unsafe { self.f.GetRasterStatus(iswapchain, prasterstatus) };
        info!("GetRasterStatus");
        r
    }

    fn SetDialogBoxMode(
        &self,
        benabledialogs: windows::Win32::Foundation::BOOL,
    ) -> windows::core::Result<()> {
        info!("SetDialogBoxMode_pre");
        let r = unsafe { self.f.SetDialogBoxMode(benabledialogs) };
        info!("SetDialogBoxMode");
        r
    }

    fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
        info!("SetGammaRamp_pre");
        let r = unsafe { self.f.SetGammaRamp(iswapchain, flags, pramp) };
        info!("SetGammaRamp");
        r
    }

    fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
        info!("GetGammaRamp_pre");
        let r = unsafe { self.f.GetGammaRamp(iswapchain, pramp) };
        info!("GetGammaRamp");
        r
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
        info!("CreateTexture_pre");
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
        info!("CreateTexture");
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
        info!("CreateVolumeTexture_pre");
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
        info!("CreateVolumeTexture");
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
        info!("CreateCubeTexture_pre");
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
        info!("CreateCubeTexture");
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
        info!("CreateVertexBuffer_pre");
        let r = unsafe {
            self.f
                .CreateVertexBuffer(length, usage, fvf, pool, ppvertexbuffer, psharedhandle)
        };
        info!("CreateVertexBuffer");
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
        info!("CreateIndexBuffer_pre");
        let r = unsafe {
            self.f
                .CreateIndexBuffer(length, usage, format, pool, ppindexbuffer, psharedhandle)
        };
        info!("CreateIndexBuffer");
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
        info!("CreateRenderTarget_pre");
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
        info!("CreateRenderTarget");
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
        info!("CreateDepthStencilSurface_pre");
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
        info!("CreateDepthStencilSurface");
        r
    }

    fn UpdateSurface(
        &self,
        psourcesurface: Option<&IDirect3DSurface9>,
        psourcerect: *const windows::Win32::Foundation::RECT,
        pdestinationsurface: Option<&IDirect3DSurface9>,
        pdestpoint: *const windows::Win32::Foundation::POINT,
    ) -> windows::core::Result<()> {
        info!("UpdateSurface_pre");
        let r = unsafe {
            self.f.UpdateSurface(
                psourcesurface,
                psourcerect,
                pdestinationsurface,
                pdestpoint,
            )
        };
        info!("UpdateSurface");
        r
    }

    fn UpdateTexture(
        &self,
        psourcetexture: Option<&IDirect3DBaseTexture9>,
        pdestinationtexture: Option<&IDirect3DBaseTexture9>,
    ) -> windows::core::Result<()> {
        info!("UpdateTexture_pre");
        let r = unsafe {
            self.f
                .UpdateTexture(psourcetexture, pdestinationtexture)
        };
        info!("UpdateTexture");
        r
    }

    fn GetRenderTargetData(
        &self,
        prendertarget: Option<&IDirect3DSurface9>,
        pdestsurface: Option<&IDirect3DSurface9>,
    ) -> windows::core::Result<()> {
        info!("GetRenderTargetData_pre");
        let r = unsafe {
            self.f
                .GetRenderTargetData(prendertarget, pdestsurface)
        };
        info!("GetRenderTargetData");
        r
    }

    fn GetFrontBufferData(
        &self,
        iswapchain: u32,
        pdestsurface: Option<&IDirect3DSurface9>,
    ) -> windows::core::Result<()> {
        info!("GetFrontBufferData_pre");
        let r = unsafe { self.f.GetFrontBufferData(iswapchain, pdestsurface) };
        info!("GetFrontBufferData");
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
        info!("StretchRect_pre");
        let r = unsafe {
            self.f.StretchRect(
                psourcesurface,
                psourcerect,
                pdestsurface,
                pdestrect,
                filter,
            )
        };
        info!("StretchRect");
        r
    }

    fn ColorFill(
        &self,
        psurface: Option<&IDirect3DSurface9>,
        prect: *const windows::Win32::Foundation::RECT,
        color: u32,
    ) -> windows::core::Result<()> {
        info!("ColorFill_pre");
        let r = unsafe { self.f.ColorFill(psurface, prect, color) };
        info!("ColorFill");
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
        info!("CreateOffscreenPlainSurface_pre");
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
        info!("CreateOffscreenPlainSurface");
        r
    }

    fn SetRenderTarget(
        &self,
        rendertargetindex: u32,
        prendertarget: Option<&IDirect3DSurface9>,
    ) -> windows::core::Result<()> {
        info!("SetRenderTarget_pre");
        let r = unsafe {
            self.f
                .SetRenderTarget(rendertargetindex, prendertarget)
        };
        info!("SetRenderTarget");
        r
    }

    fn GetRenderTarget(&self, rendertargetindex: u32) -> windows::core::Result<IDirect3DSurface9> {
        info!("GetRenderTarget_pre");
        let r = unsafe { self.f.GetRenderTarget(rendertargetindex) };
        info!("GetRenderTarget");
        r
    }

    fn SetDepthStencilSurface(
        &self,
        pnewzstencil: Option<&IDirect3DSurface9>,
    ) -> windows::core::Result<()> {
        info!("SetDepthStencilSurface_pre");
        let r = unsafe { self.f.SetDepthStencilSurface(pnewzstencil) };
        info!("SetDepthStencilSurface");
        r
    }

    fn GetDepthStencilSurface(&self) -> windows::core::Result<IDirect3DSurface9> {
        info!("GetDepthStencilSurface_pre");
        let r = unsafe { self.f.GetDepthStencilSurface() };
        info!("GetDepthStencilSurface");
        r
    }

    fn BeginScene(&self) -> windows::core::Result<()> {
        info!("BeginScene_pre");
        let r = unsafe { self.f.BeginScene() };
        info!("BeginScene");
        r
    }

    fn EndScene(&self) -> windows::core::Result<()> {
        info!("EndScene_pre");
        let r = unsafe { self.f.EndScene() };
        info!("EndScene");
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
        info!("Clear_pre");
        let r = unsafe { self.f.Clear(count, prects, flags, color, z, stencil) };
        info!("Clear");
        r
    }

    fn SetTransform(
        &self,
        state: D3DTRANSFORMSTATETYPE,
        pmatrix: *const windows::Foundation::Numerics::Matrix4x4,
    ) -> windows::core::Result<()> {
        info!("SetTransform_pre");
        let r = unsafe { self.f.SetTransform(state, pmatrix) };
        info!("SetTransform");
        r
    }

    fn GetTransform(
        &self,
        state: D3DTRANSFORMSTATETYPE,
        pmatrix: *mut windows::Foundation::Numerics::Matrix4x4,
    ) -> windows::core::Result<()> {
        info!("GetTransform_pre");
        let r = unsafe { self.f.GetTransform(state, pmatrix) };
        info!("GetTransform");
        r
    }

    fn MultiplyTransform(
        &self,
        param0: D3DTRANSFORMSTATETYPE,
        param1: *const windows::Foundation::Numerics::Matrix4x4,
    ) -> windows::core::Result<()> {
        info!("MultiplyTransform_pre");
        let r = unsafe { self.f.MultiplyTransform(param0, param1) };
        info!("MultiplyTransform");
        r
    }

    fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> windows::core::Result<()> {
        info!("SetViewport_pre");
        let r = unsafe { self.f.SetViewport(pviewport) };
        info!("SetViewport");
        r
    }

    fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> windows::core::Result<()> {
        info!("GetViewport_pre");
        let r = unsafe { self.f.GetViewport(pviewport) };
        info!("GetViewport");
        r
    }

    fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> windows::core::Result<()> {
        info!("SetMaterial_pre");
        let r = unsafe { self.f.SetMaterial(pmaterial) };
        info!("SetMaterial");
        r
    }

    fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> windows::core::Result<()> {
        info!("GetMaterial_pre");
        let r = unsafe { self.f.GetMaterial(pmaterial) };
        info!("GetMaterial");
        r
    }

    fn SetLight(&self, index: u32, param1: *const D3DLIGHT9) -> windows::core::Result<()> {
        info!("SetLight_pre");
        let r = unsafe { self.f.SetLight(index, param1) };
        info!("SetLight");
        r
    }

    fn GetLight(&self, index: u32, param1: *mut D3DLIGHT9) -> windows::core::Result<()> {
        info!("GetLight_pre");
        let r = unsafe { self.f.GetLight(index, param1) };
        info!("GetLight");
        r
    }

    fn LightEnable(
        &self,
        index: u32,
        enable: windows::Win32::Foundation::BOOL,
    ) -> windows::core::Result<()> {
        info!("LightEnable_pre");
        let r = unsafe { self.f.LightEnable(index, enable) };
        info!("LightEnable");
        r
    }

    fn GetLightEnable(
        &self,
        index: u32,
        penable: *mut windows::Win32::Foundation::BOOL,
    ) -> windows::core::Result<()> {
        info!("GetLightEnable_pre");
        let r = unsafe { self.f.GetLightEnable(index, penable) };
        info!("GetLightEnable");
        r
    }

    fn SetClipPlane(&self, index: u32, pplane: *const f32) -> windows::core::Result<()> {
        info!("SetClipPlane_pre");
        let r = unsafe { self.f.SetClipPlane(index, pplane) };
        info!("SetClipPlane");
        r
    }

    fn GetClipPlane(&self, index: u32, pplane: *mut f32) -> windows::core::Result<()> {
        info!("GetClipPlane_pre");
        let r = unsafe { self.f.GetClipPlane(index, pplane) };
        info!("GetClipPlane");
        r
    }

    fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: u32) -> windows::core::Result<()> {
        info!("SetRenderState_pre");
        let r = unsafe { self.f.SetRenderState(state, value) };
        info!("SetRenderState");
        r
    }

    fn GetRenderState(
        &self,
        state: D3DRENDERSTATETYPE,
        pvalue: *mut u32,
    ) -> windows::core::Result<()> {
        info!("GetRenderState_pre");
        let r = unsafe { self.f.GetRenderState(state, pvalue) };
        info!("GetRenderState");
        r
    }

    fn CreateStateBlock(
        &self,
        stateblock_type: D3DSTATEBLOCKTYPE,
    ) -> windows::core::Result<IDirect3DStateBlock9> {
        info!("CreateStateBlock_pre");
        let r = unsafe { self.f.CreateStateBlock(stateblock_type) };
        info!("CreateStateBlock");
        r
    }

    fn BeginStateBlock(&self) -> windows::core::Result<()> {
        info!("BeginStateBlock_pre");
        let r = unsafe { self.f.BeginStateBlock() };
        info!("BeginStateBlock");
        r
    }

    fn EndStateBlock(&self) -> windows::core::Result<IDirect3DStateBlock9> {
        info!("EndStateBlock_pre");
        let r = unsafe { self.f.EndStateBlock() };
        info!("EndStateBlock");
        r
    }

    fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> windows::core::Result<()> {
        info!("SetClipStatus_pre");
        let r = unsafe { self.f.SetClipStatus(pclipstatus) };
        info!("SetClipStatus");
        r
    }

    fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> windows::core::Result<()> {
        info!("GetClipStatus_pre");
        let r = unsafe { self.f.GetClipStatus(pclipstatus) };
        info!("GetClipStatus");
        r
    }

    fn GetTexture(&self, stage: u32) -> windows::core::Result<IDirect3DBaseTexture9> {
        info!("GetTexture_pre");
        let r = unsafe { self.f.GetTexture(stage) };
        info!("GetTexture");
        r
    }

    fn SetTexture(
        &self,
        stage: u32,
        ptexture: Option<&IDirect3DBaseTexture9>,
    ) -> windows::core::Result<()> {
        info!("SetTexture_pre");
        let r = unsafe { self.f.SetTexture(stage, ptexture) };
        info!("SetTexture");
        r
    }

    fn GetTextureStageState(
        &self,
        stage: u32,
        texstage_type: D3DTEXTURESTAGESTATETYPE,
        pvalue: *mut u32,
    ) -> windows::core::Result<()> {
        info!("GetTextureStageState_pre");
        let r = unsafe { self.f.GetTextureStageState(stage, texstage_type, pvalue) };
        info!("GetTextureStageState");
        r
    }

    fn SetTextureStageState(
        &self,
        stage: u32,
        texstage_type: D3DTEXTURESTAGESTATETYPE,
        value: u32,
    ) -> windows::core::Result<()> {
        info!("SetTextureStageState_pre");
        let r = unsafe { self.f.SetTextureStageState(stage, texstage_type, value) };
        info!("SetTextureStageState");
        r
    }

    fn GetSamplerState(
        &self,
        sampler: u32,
        samplerstate_type: D3DSAMPLERSTATETYPE,
        pvalue: *mut u32,
    ) -> windows::core::Result<()> {
        info!("GetSamplerState_pre");
        let r = unsafe { self.f.GetSamplerState(sampler, samplerstate_type, pvalue) };
        info!("GetSamplerState");
        r
    }

    fn SetSamplerState(
        &self,
        sampler: u32,
        samplerstate_type: D3DSAMPLERSTATETYPE,
        value: u32,
    ) -> windows::core::Result<()> {
        info!("SetSamplerState_pre");
        let r = unsafe { self.f.SetSamplerState(sampler, samplerstate_type, value) };
        info!("SetSamplerState");
        r
    }

    fn ValidateDevice(&self, pnumpasses: *mut u32) -> windows::core::Result<()> {
        info!("ValidateDevice_pre");
        let r = unsafe { self.f.ValidateDevice(pnumpasses) };
        info!("ValidateDevice");
        r
    }

    fn SetPaletteEntries(
        &self,
        palettenumber: u32,
        pentries: *const windows::Win32::Graphics::Gdi::PALETTEENTRY,
    ) -> windows::core::Result<()> {
        info!("SetPaletteEntries_pre");
        let r = unsafe { self.f.SetPaletteEntries(palettenumber, pentries) };
        info!("SetPaletteEntries");
        r
    }

    fn GetPaletteEntries(
        &self,
        palettenumber: u32,
        pentries: *mut windows::Win32::Graphics::Gdi::PALETTEENTRY,
    ) -> windows::core::Result<()> {
        info!("GetPaletteEntries_pre");
        let r = unsafe { self.f.GetPaletteEntries(palettenumber, pentries) };
        info!("GetPaletteEntries");
        r
    }

    fn SetCurrentTexturePalette(&self, palettenumber: u32) -> windows::core::Result<()> {
        info!("SetCurrentTexturePalette_pre");
        let r = unsafe { self.f.SetCurrentTexturePalette(palettenumber) };
        info!("SetCurrentTexturePalette");
        r
    }

    fn GetCurrentTexturePalette(&self, palettenumber: *mut u32) -> windows::core::Result<()> {
        info!("GetCurrentTexturePalette_pre");
        let r = unsafe { self.f.GetCurrentTexturePalette(palettenumber) };
        info!("GetCurrentTexturePalette");
        r
    }

    fn SetScissorRect(
        &self,
        prect: *const windows::Win32::Foundation::RECT,
    ) -> windows::core::Result<()> {
        info!("SetScissorRect_pre");
        let r = unsafe { self.f.SetScissorRect(prect) };
        info!("SetScissorRect");
        r
    }

    fn GetScissorRect(
        &self,
        prect: *mut windows::Win32::Foundation::RECT,
    ) -> windows::core::Result<()> {
        info!("GetScissorRect_pre");
        let r = unsafe { self.f.GetScissorRect(prect) };
        info!("GetScissorRect");
        r
    }

    fn SetSoftwareVertexProcessing(
        &self,
        bsoftware: windows::Win32::Foundation::BOOL,
    ) -> windows::core::Result<()> {
        info!("SetSoftwareVertexProcessing_pre");
        let r = unsafe { self.f.SetSoftwareVertexProcessing(bsoftware) };
        info!("SetSoftwareVertexProcessing");
        r
    }

    fn GetSoftwareVertexProcessing(&self) -> windows::Win32::Foundation::BOOL {
        info!("GetSoftwareVertexProcessing_pre");
        let r = unsafe { self.f.GetSoftwareVertexProcessing() };
        info!("GetSoftwareVertexProcessing");
        r
    }

    fn SetNPatchMode(&self, nsegments: f32) -> windows::core::Result<()> {
        info!("SetNPatchMode_pre");
        let r = unsafe { self.f.SetNPatchMode(nsegments) };
        info!("SetNPatchMode");
        r
    }

    fn GetNPatchMode(&self) -> f32 {
        info!("GetNPatchMode_pre");
        let r = unsafe { self.f.GetNPatchMode() };
        info!("GetNPatchMode");
        r
    }

    fn DrawPrimitive(
        &self,
        primitivetype: D3DPRIMITIVETYPE,
        startvertex: u32,
        primitivecount: u32,
    ) -> windows::core::Result<()> {
        info!("DrawPrimitive_pre");
        let r = unsafe {
            self.f
                .DrawPrimitive(primitivetype, startvertex, primitivecount)
        };
        info!("DrawPrimitive");
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
        info!("DrawIndexedPrimitive_pre");
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
        info!("DrawIndexedPrimitive");
        r
    }

    fn DrawPrimitiveUP(
        &self,
        primitivetype: D3DPRIMITIVETYPE,
        primitivecount: u32,
        pvertexstreamzerodata: *const core::ffi::c_void,
        vertexstreamzerostride: u32,
    ) -> windows::core::Result<()> {
        info!("DrawPrimitiveUP_pre");
        let r = unsafe {
            self.f.DrawPrimitiveUP(
                primitivetype,
                primitivecount,
                pvertexstreamzerodata,
                vertexstreamzerostride,
            )
        };
        info!("DrawPrimitiveUP");
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
        info!("DrawIndexedPrimitiveUP_pre");
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
        info!("DrawIndexedPrimitiveUP");
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
        info!("ProcessVertices_pre");
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
        info!("ProcessVertices");
        r
    }

    fn CreateVertexDeclaration(
        &self,
        pvertexelements: *const D3DVERTEXELEMENT9,
    ) -> windows::core::Result<IDirect3DVertexDeclaration9> {
        info!("CreateVertexDeclaration_pre");
        let r = unsafe { self.f.CreateVertexDeclaration(pvertexelements) };
        info!("CreateVertexDeclaration");
        r
    }

    fn SetVertexDeclaration(
        &self,
        pdecl: Option<&IDirect3DVertexDeclaration9>,
    ) -> windows::core::Result<()> {
        info!("SetVertexDeclaration_pre");
        let r = unsafe { self.f.SetVertexDeclaration(pdecl) };
        info!("SetVertexDeclaration");
        r
    }

    fn GetVertexDeclaration(&self) -> windows::core::Result<IDirect3DVertexDeclaration9> {
        info!("GetVertexDeclaration_pre");
        let r = unsafe { self.f.GetVertexDeclaration() };
        info!("GetVertexDeclaration");
        r
    }

    fn SetFVF(&self, fvf: u32) -> windows::core::Result<()> {
        info!("SetFVF_pre");
        let r = unsafe { self.f.SetFVF(fvf) };
        info!("SetFVF");
        r
    }

    fn GetFVF(&self, pfvf: *mut u32) -> windows::core::Result<()> {
        info!("GetFVF_pre");
        let r = unsafe { self.f.GetFVF(pfvf) };
        info!("GetFVF");
        r
    }

    fn CreateVertexShader(
        &self,
        pfunction: *const u32,
    ) -> windows::core::Result<IDirect3DVertexShader9> {
        info!("CreateVertexShader_pre");
        let r = unsafe { self.f.CreateVertexShader(pfunction) };
        info!("CreateVertexShader");
        r
    }

    fn SetVertexShader(
        &self,
        pshader: Option<&IDirect3DVertexShader9>,
    ) -> windows::core::Result<()> {
        info!("SetVertexShader_pre");
        let r = unsafe { self.f.SetVertexShader(pshader) };
        info!("SetVertexShader");
        r
    }

    fn GetVertexShader(&self) -> windows::core::Result<IDirect3DVertexShader9> {
        info!("GetVertexShader_pre");
        let r = unsafe { self.f.GetVertexShader() };
        info!("GetVertexShader");
        r
    }

    fn SetVertexShaderConstantF(
        &self,
        startregister: u32,
        pconstantdata: *const f32,
        vector4fcount: u32,
    ) -> windows::core::Result<()> {
        info!("SetVertexShaderConstantF_pre");
        let r = unsafe {
            self.f
                .SetVertexShaderConstantF(startregister, pconstantdata, vector4fcount)
        };
        info!("SetVertexShaderConstantF");
        r
    }

    fn GetVertexShaderConstantF(
        &self,
        startregister: u32,
        pconstantdata: *mut f32,
        vector4fcount: u32,
    ) -> windows::core::Result<()> {
        info!("GetVertexShaderConstantF_pre");
        let r = unsafe {
            self.f
                .GetVertexShaderConstantF(startregister, pconstantdata, vector4fcount)
        };
        info!("GetVertexShaderConstantF");
        r
    }

    fn SetVertexShaderConstantI(
        &self,
        startregister: u32,
        pconstantdata: *const i32,
        vector4icount: u32,
    ) -> windows::core::Result<()> {
        info!("SetVertexShaderConstantI_pre");
        let r = unsafe {
            self.f
                .SetVertexShaderConstantI(startregister, pconstantdata, vector4icount)
        };
        info!("SetVertexShaderConstantI");
        r
    }

    fn GetVertexShaderConstantI(
        &self,
        startregister: u32,
        pconstantdata: *mut i32,
        vector4icount: u32,
    ) -> windows::core::Result<()> {
        info!("GetVertexShaderConstantI_pre");
        let r = unsafe {
            self.f
                .GetVertexShaderConstantI(startregister, pconstantdata, vector4icount)
        };
        info!("GetVertexShaderConstantI");
        r
    }

    fn SetVertexShaderConstantB(
        &self,
        startregister: u32,
        pconstantdata: *const windows::Win32::Foundation::BOOL,
        boolcount: u32,
    ) -> windows::core::Result<()> {
        info!("SetVertexShaderConstantB_pre");
        let r = unsafe {
            self.f
                .SetVertexShaderConstantB(startregister, pconstantdata, boolcount)
        };
        info!("SetVertexShaderConstantB");
        r
    }

    fn GetVertexShaderConstantB(
        &self,
        startregister: u32,
        pconstantdata: *mut windows::Win32::Foundation::BOOL,
        boolcount: u32,
    ) -> windows::core::Result<()> {
        info!("GetVertexShaderConstantB_pre");
        let r = unsafe {
            self.f
                .GetVertexShaderConstantB(startregister, pconstantdata, boolcount)
        };
        info!("GetVertexShaderConstantB");
        r
    }

    fn SetStreamSource(
        &self,
        streamnumber: u32,
        pstreamdata: Option<&IDirect3DVertexBuffer9>,
        offsetinbytes: u32,
        stride: u32,
    ) -> windows::core::Result<()> {
        info!("SetStreamSource_pre");
        let r = unsafe {
            self.f
                .SetStreamSource(streamnumber, pstreamdata, offsetinbytes, stride)
        };
        info!("SetStreamSource");
        r
    }

    fn GetStreamSource(
        &self,
        streamnumber: u32,
        ppstreamdata: *mut Option<IDirect3DVertexBuffer9>,
        poffsetinbytes: *mut u32,
        pstride: *mut u32,
    ) -> windows::core::Result<()> {
        info!("GetStreamSource_pre");
        let r = unsafe {
            self.f
                .GetStreamSource(streamnumber, ppstreamdata, poffsetinbytes, pstride)
        };
        info!("GetStreamSource");
        r
    }

    fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> windows::core::Result<()> {
        info!("SetStreamSourceFreq_pre");
        let r = unsafe { self.f.SetStreamSourceFreq(streamnumber, setting) };
        info!("SetStreamSourceFreq");
        r
    }

    fn GetStreamSourceFreq(
        &self,
        streamnumber: u32,
        psetting: *mut u32,
    ) -> windows::core::Result<()> {
        info!("GetStreamSourceFreq_pre");
        let r = unsafe { self.f.GetStreamSourceFreq(streamnumber, psetting) };
        info!("GetStreamSourceFreq");
        r
    }

    fn SetIndices(&self, pindexdata: Option<&IDirect3DIndexBuffer9>) -> windows::core::Result<()> {
        info!("SetIndices_pre");
        let r = unsafe { self.f.SetIndices(pindexdata) };
        info!("SetIndices");
        r
    }

    fn GetIndices(&self) -> windows::core::Result<IDirect3DIndexBuffer9> {
        info!("GetIndices_pre");
        let r = unsafe { self.f.GetIndices() };
        info!("GetIndices");
        r
    }

    fn CreatePixelShader(
        &self,
        pfunction: *const u32,
    ) -> windows::core::Result<IDirect3DPixelShader9> {
        info!("CreatePixelShader_pre");
        let r = unsafe { self.f.CreatePixelShader(pfunction) };
        info!("CreatePixelShader");
        r
    }

    fn SetPixelShader(&self, pshader: Option<&IDirect3DPixelShader9>) -> windows::core::Result<()> {
        info!("SetPixelShader_pre");
        let r = unsafe { self.f.SetPixelShader(pshader) };
        info!("SetPixelShader");
        r
    }

    fn GetPixelShader(&self) -> windows::core::Result<IDirect3DPixelShader9> {
        info!("GetPixelShader_pre");
        let r = unsafe { self.f.GetPixelShader() };
        info!("GetPixelShader");
        r
    }

    fn SetPixelShaderConstantF(
        &self,
        startregister: u32,
        pconstantdata: *const f32,
        vector4fcount: u32,
    ) -> windows::core::Result<()> {
        info!("SetPixelShaderConstantF_pre");
        let r = unsafe {
            self.f
                .SetPixelShaderConstantF(startregister, pconstantdata, vector4fcount)
        };
        info!("SetPixelShaderConstantF");
        r
    }

    fn GetPixelShaderConstantF(
        &self,
        startregister: u32,
        pconstantdata: *mut f32,
        vector4fcount: u32,
    ) -> windows::core::Result<()> {
        info!("GetPixelShaderConstantF_pre");
        let r = unsafe {
            self.f
                .GetPixelShaderConstantF(startregister, pconstantdata, vector4fcount)
        };
        info!("GetPixelShaderConstantF");
        r
    }

    fn SetPixelShaderConstantI(
        &self,
        startregister: u32,
        pconstantdata: *const i32,
        vector4icount: u32,
    ) -> windows::core::Result<()> {
        info!("SetPixelShaderConstantI_pre");
        let r = unsafe {
            self.f
                .SetPixelShaderConstantI(startregister, pconstantdata, vector4icount)
        };
        info!("SetPixelShaderConstantI");
        r
    }

    fn GetPixelShaderConstantI(
        &self,
        startregister: u32,
        pconstantdata: *mut i32,
        vector4icount: u32,
    ) -> windows::core::Result<()> {
        info!("GetPixelShaderConstantI_pre");
        let r = unsafe {
            self.f
                .GetPixelShaderConstantI(startregister, pconstantdata, vector4icount)
        };
        info!("GetPixelShaderConstantI");
        r
    }

    fn SetPixelShaderConstantB(
        &self,
        startregister: u32,
        pconstantdata: *const windows::Win32::Foundation::BOOL,
        boolcount: u32,
    ) -> windows::core::Result<()> {
        info!("SetVertexShaderConstantB_pre");
        let r = unsafe {
            self.f
                .SetPixelShaderConstantB(startregister, pconstantdata, boolcount)
        };
        info!("SetVertexShaderConstantB");
        r
    }

    fn GetPixelShaderConstantB(
        &self,
        startregister: u32,
        pconstantdata: *mut windows::Win32::Foundation::BOOL,
        boolcount: u32,
    ) -> windows::core::Result<()> {
        info!("GetPixelShaderConstantB_pre");
        let r = unsafe {
            self.f
                .GetPixelShaderConstantB(startregister, pconstantdata, boolcount)
        };
        info!("GetPixelShaderConstantB");
        r
    }

    fn DrawRectPatch(
        &self,
        handle: u32,
        pnumsegs: *const f32,
        prectpatchinfo: *const D3DRECTPATCH_INFO,
    ) -> windows::core::Result<()> {
        info!("DrawRectPatch_pre");
        let r = unsafe { self.f.DrawRectPatch(handle, pnumsegs, prectpatchinfo) };
        info!("DrawRectPatch");
        r
    }

    fn DrawTriPatch(
        &self,
        handle: u32,
        pnumsegs: *const f32,
        ptripatchinfo: *const D3DTRIPATCH_INFO,
    ) -> windows::core::Result<()> {
        info!("DrawTriPatch_pre");
        let r = unsafe { self.f.DrawTriPatch(handle, pnumsegs, ptripatchinfo) };
        info!("DrawTriPatch");
        r
    }

    fn DeletePatch(&self, handle: u32) -> windows::core::Result<()> {
        info!("DeletePatch_pre");
        let r = unsafe { self.f.DeletePatch(handle) };
        info!("DeletePatch");
        r
    }

    // Last
    fn CreateQuery(&self, q_type: D3DQUERYTYPE) -> windows::core::Result<IDirect3DQuery9> {
        info!("CreateQuery_pre");
        let r = unsafe { self.f.CreateQuery(q_type) };
        info!("CreateQuery");
        r
    }
}
