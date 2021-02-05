
use gl;
use pathfinder_gl::{GLDevice, GLVersion};
use std::ffi::CString;
use pathfinder_renderer::gpu::options::{DestFramebuffer};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_geometry::rect::RectI;
use libc::c_void;

use crate::{
    PFGLFunctionLoader, PFGLVersion, PFGLDeviceRef, PFGLDestFramebufferRef, 
    PFVector2I, PFResourceLoaderRef, PFRendererMode, PFRendererOptions, 
    PFBuildOptionsRef, PFSceneProxyRef, PFGLRendererRef
};

pub const PF_GL_VERSION_GL3:    u8 = 0;
pub const PF_GL_VERSION_GLES3:  u8 = 1;
pub const PF_GL_VERSION_GL4:    u8 = 2;

#[no_mangle]
pub unsafe extern "C" fn PFGLLoadWith(loader: PFGLFunctionLoader, userdata: *mut c_void) {
    gl::load_with(|name| {
        let name = CString::new(name).unwrap();
        loader(name.as_ptr(), userdata)
    });
}

#[no_mangle]
pub unsafe extern "C" fn PFGLDeviceCreate(version: PFGLVersion, default_framebuffer: u32)
                                          -> PFGLDeviceRef {
    let version = match version {
        PF_GL_VERSION_GL3   => GLVersion::GL3,
        PF_GL_VERSION_GLES3 => GLVersion::GLES3,
        PF_GL_VERSION_GL4   => GLVersion::GL4,
        _ => panic!("Invalid Pathfinder OpenGL version!"),
    };
    Box::into_raw(Box::new(GLDevice::new(version, default_framebuffer)))
}

#[no_mangle]
pub unsafe extern "C" fn PFGLDeviceDestroy(device: PFGLDeviceRef) {
    drop(Box::from_raw(device))
}

// `gpu`

#[no_mangle]
pub unsafe extern "C" fn PFGLDestFramebufferCreateFullWindow(window_size: *const PFVector2I)
                                                             -> PFGLDestFramebufferRef {
    Box::into_raw(Box::new(DestFramebuffer::full_window((*window_size).to_rust())))
}

#[no_mangle]
pub unsafe extern "C" fn PFGLDestFramebufferCreate(window_size: *const PFVector2I, viewport_position: *const PFVector2I, viewport_size: *const PFVector2I)
                                                             -> PFGLDestFramebufferRef {
    Box::into_raw(Box::new(
        DestFramebuffer::Default{
			viewport: RectI::new(
			(*viewport_size).to_rust(), 
			(*viewport_position).to_rust()),
			
			window_size: (*window_size).to_rust()
		}
	))
}

#[no_mangle]
pub unsafe extern "C" fn PFGLDestFramebufferDestroy(dest_framebuffer: PFGLDestFramebufferRef) {
    drop(Box::from_raw(dest_framebuffer))
}

/// This function takes ownership of and automatically takes responsibility for destroying `device`
/// and `dest_framebuffer`. However, it does not take ownership of `resources`; therefore, if you
/// created the resource loader, you must destroy it yourself to avoid a memory leak.
#[no_mangle]
pub unsafe extern "C" fn PFGLRendererCreate(device: PFGLDeviceRef,
                                            resources: PFResourceLoaderRef,
                                            mode: *const PFRendererMode,
                                            options: *const PFRendererOptions)
                                            -> PFGLRendererRef {
    Box::into_raw(Box::new(Renderer::new(*Box::from_raw(device),
                                         &*((*resources).0),
                                         (*mode).to_rust(),
                                         (*options).to_rust())))
}

#[no_mangle]
pub unsafe extern "C" fn PFGLRendererReplaceDestFramebuffer(renderer: PFGLRendererRef, window_size: *const PFVector2I, viewport_position: *const PFVector2I, viewport_size: *const PFVector2I) {
    (*renderer).options_mut().dest = 
        DestFramebuffer::Default{
		viewport: RectI::new((*viewport_position).to_rust(), (*viewport_size).to_rust()),
		window_size: (*window_size).to_rust(),
		};
		
	(*renderer).dest_framebuffer_size_changed();
}

#[no_mangle]
pub unsafe extern "C" fn PFGLRendererDestroy(renderer: PFGLRendererRef) {
    drop(Box::from_raw(renderer))
}

#[no_mangle]
pub unsafe extern "C" fn PFGLRendererGetDevice(renderer: PFGLRendererRef) -> PFGLDeviceRef {
    (*renderer).device_mut()
}

/// This function does not take ownership of `renderer` or `build_options`. Therefore, if you
/// created the renderer and/or options, you must destroy them yourself to avoid a leak.
#[no_mangle]
pub unsafe extern "C" fn PFSceneProxyBuildAndRenderGL(scene_proxy: PFSceneProxyRef,
                                                      renderer: PFGLRendererRef,
                                                      build_options: PFBuildOptionsRef) {
    (*scene_proxy).build_and_render(&mut *renderer, (*build_options).clone())
}