#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
use crate::{
    PFMetalDestFramebufferRef, PFVector2I, PFMetalDeviceRef, PFResourceLoaderRef,
    PFRendererMode, PFRendererOptions, PFMetalRendererRef, PFSceneProxyRef,
    IOSurfaceRef, CoreAnimationDrawableRef, NativeMetalDeviceRef
};


#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalDestFramebufferCreateFullWindow(window_size: *const PFVector2I)
                                                                -> PFMetalDestFramebufferRef {
    Box::into_raw(Box::new(DestFramebuffer::full_window((*window_size).to_rust())))
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalDestFramebufferDestroy(dest_framebuffer:
                                                       PFMetalDestFramebufferRef) {
    drop(Box::from_raw(dest_framebuffer))
}

/// This function takes ownership of and automatically takes responsibility for destroying `device`
/// and `dest_framebuffer`. However, it does not take ownership of `resources`; therefore, if you
/// created the resource loader, you must destroy it yourself to avoid a memory leak.
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalRendererCreate(device: PFMetalDeviceRef,
                                               resources: PFResourceLoaderRef,
                                               mode: *const PFRendererMode,
                                               options: *const PFRendererOptions)
                                               -> PFMetalRendererRef {
    Box::into_raw(Box::new(Renderer::new(*Box::from_raw(device),
                                         &*((*resources).0),
                                         (*mode).to_rust(),
                                         (*options).to_rust())))
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalRendererDestroy(renderer: PFMetalRendererRef) {
    drop(Box::from_raw(renderer))
}

/// Returns a reference to the Metal device in the renderer.
///
/// This reference remains valid as long as the device is alive.
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalRendererGetDevice(renderer: PFMetalRendererRef)
                                                  -> PFMetalDeviceRef {
    (*renderer).device_mut()
}



/// This function does not take ownership of `renderer` or `build_options`. Therefore, if you
/// created the renderer and/or options, you must destroy them yourself to avoid a leak.
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFSceneProxyBuildAndRenderMetal(scene_proxy: PFSceneProxyRef,
                                                         renderer: PFMetalRendererRef,
                                                         build_options: PFBuildOptionsRef) {
    (*scene_proxy).build_and_render(&mut *renderer, (*build_options).clone())
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalDeviceCreateWithIOSurface(metal_device: &NativeMetalDeviceRef,
                                                          io_surface: IOSurfaceRef)
                                                          -> PFMetalDeviceRef {
    Box::into_raw(Box::new(MetalDevice::new(metal_device, io_surface)))
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalDeviceCreateWithDrawable(metal_device: &NativeMetalDeviceRef,
                                                         ca_drawable: &CoreAnimationDrawableRef)
                                                         -> PFMetalDeviceRef {
    Box::into_raw(Box::new(MetalDevice::new(metal_device, ca_drawable)))
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalDeviceSwapIOSurface(device: PFMetalDeviceRef,
                                                    new_io_surface: IOSurfaceRef) {
    drop((*device).swap_texture(new_io_surface))
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalDeviceSwapDrawable(device: PFMetalDeviceRef,
                                                   new_ca_drawable: &CoreAnimationDrawableRef) {
    drop((*device).swap_texture(new_ca_drawable))
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalDevicePresentDrawable(device: PFMetalDeviceRef,
                                                      ca_drawable: &CoreAnimationDrawableRef) {
    (*device).present_drawable(ca_drawable)
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "C" fn PFMetalDeviceDestroy(device: PFMetalDeviceRef) {
    drop(Box::from_raw(device))
}