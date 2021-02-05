
#![allow(non_snake_case)]

use crate::{
    PFTransform2F, PFRenderTransformRef, PFPerspective,
    PFBuildOptionsRef, PFVector2F, PFSceneRef, PFSceneProxyRef,
    PFRendererLevel, PFRendererOptions, PFRendererMode
};

use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererLevel};
use pathfinder_renderer::gpu::options::{RendererMode, RendererOptions};
use pathfinder_renderer::options::{BuildOptions, RenderTransform};
use pathfinder_gpu::Device;

pub const PF_RENDERER_OPTIONS_FLAGS_HAS_BACKGROUND_COLOR: u8 = 0x1;
pub const PF_RENDERER_OPTIONS_FLAGS_SHOW_DEBUG_UI: u8 = 0x2;

pub const PF_RENDERER_LEVEL_D3D9: u8 = 0x1;
pub const PF_RENDERER_LEVEL_D3D11: u8 = 0x2;

#[no_mangle]
pub unsafe extern "C" fn PFRenderTransformCreate2D(transform: *const PFTransform2F)
                                                   -> PFRenderTransformRef {
    Box::into_raw(Box::new(RenderTransform::Transform2D((*transform).to_rust())))
}

#[no_mangle]
pub unsafe extern "C" fn PFRenderTransformCreatePerspective(perspective: *const PFPerspective)
                                                            -> PFRenderTransformRef {
    Box::into_raw(Box::new(RenderTransform::Perspective((*perspective).to_rust())))
}

#[no_mangle]
pub unsafe extern "C" fn PFRenderTransformDestroy(transform: PFRenderTransformRef) {
    drop(Box::from_raw(transform))
}

#[no_mangle]
pub unsafe extern "C" fn PFBuildOptionsCreate() -> PFBuildOptionsRef {
    Box::into_raw(Box::new(BuildOptions::default()))
}

#[no_mangle]
pub unsafe extern "C" fn PFBuildOptionsDestroy(options: PFBuildOptionsRef) {
    drop(Box::from_raw(options))
}

/// Consumes the transform.
#[no_mangle]
pub unsafe extern "C" fn PFBuildOptionsSetTransform(options: PFBuildOptionsRef,
                                                    transform: PFRenderTransformRef) {
    (*options).transform = *Box::from_raw(transform)
}

#[no_mangle]
pub unsafe extern "C" fn PFBuildOptionsSetTransform2D(options: PFBuildOptionsRef, transform: *const PFTransform2F) {
    (*options).transform = RenderTransform::Transform2D((*transform).to_rust());
}

#[no_mangle]
pub unsafe extern "C" fn PFBuildOptionsSetTransformPerspective(options: PFBuildOptionsRef, transform: *const PFPerspective) {
    (*options).transform = RenderTransform::Perspective((*transform).to_rust());
}

#[no_mangle]
pub unsafe extern "C" fn PFBuildOptionsSetDilation(options: PFBuildOptionsRef,
                                                   dilation: *const PFVector2F) {
    (*options).dilation = (*dilation).to_rust()
}

#[no_mangle]
pub unsafe extern "C" fn PFBuildOptionsSetSubpixelAAEnabled(options: PFBuildOptionsRef,
                                                            subpixel_aa_enabled: bool) {
    (*options).subpixel_aa_enabled = subpixel_aa_enabled
}

#[no_mangle]
pub unsafe extern "C" fn PFSceneDestroy(scene: PFSceneRef) {
    drop(Box::from_raw(scene))
}

#[no_mangle]
pub unsafe extern "C" fn PFSceneProxyCreateFromSceneAndRayonExecutor(scene: PFSceneRef,
                                                                     level: PFRendererLevel)
                                                                     -> PFSceneProxyRef {
    Box::into_raw(Box::new(SceneProxy::from_scene(*Box::from_raw(scene),
                                                  to_rust_renderer_level(level),
                                                  RayonExecutor)))
}

#[no_mangle]
pub unsafe extern "C" fn PFSceneProxyDestroy(scene_proxy: PFSceneProxyRef) {
    drop(Box::from_raw(scene_proxy))
}

// Helpers

fn to_rust_renderer_level(level: PFRendererLevel) -> RendererLevel {
    match level {
        PF_RENDERER_LEVEL_D3D9  => RendererLevel::D3D9,
        PF_RENDERER_LEVEL_D3D11 => RendererLevel::D3D11,
        _                       => panic!("Invalid Pathfinder renderer level!"),
    }
}

impl PFRendererMode {
    pub fn to_rust(&self) -> RendererMode {
        RendererMode {
            level: to_rust_renderer_level(self.level),
        }
    }
}

impl PFRendererOptions {
    pub fn to_rust<D>(&self) -> RendererOptions<D> where D: Device {
        let has_background_color = self.flags & PF_RENDERER_OPTIONS_FLAGS_HAS_BACKGROUND_COLOR;
        let show_debug_ui = (self.flags & PF_RENDERER_OPTIONS_FLAGS_SHOW_DEBUG_UI) != 0;
        unsafe {
            RendererOptions {
                background_color: if has_background_color != 0 {
                    Some(self.background_color.to_rust())
                } else {
                    None
                },
                dest: *Box::from_raw(self.dest as *mut DestFramebuffer<D>),
                show_debug_ui,
            }
        }
    }
}