// pathfinder/c/src/lib.rs
//
// Copyright 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! C bindings to Pathfinder.
#![allow(non_snake_case)]

mod canvas;
mod fill;
mod resources;
mod path;
mod renderer;

mod gl_backend;

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
mod metal_backend;

use font_kit::handle::Handle;
use pathfinder_canvas::{CanvasFontContext, CanvasRenderingContext2D, FillStyle};
use pathfinder_canvas::{Path2D};
use pathfinder_color::{ColorF, ColorU};
use pathfinder_geometry::rect::{RectF, RectI};
use pathfinder_geometry::transform2d::{Matrix2x2F, Transform2F};
use pathfinder_geometry::transform3d::{Perspective, Transform4F};
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use pathfinder_gl::{GLDevice};
use pathfinder_resources::ResourceLoader;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer};
use pathfinder_renderer::scene::Scene;
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::{BuildOptions, RenderTransform};
use pathfinder_simd::default::F32x4;
use pathfinder_svg::SVGScene;
use std::os::raw::{c_char, c_void};
use std::path::PathBuf;
use std::ptr;
use std::slice;
use std::str;
use usvg::{Options, Tree};

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
use io_surface::IOSurfaceRef;
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
use metal::{self, CoreAnimationDrawableRef, DeviceRef as NativeMetalDeviceRef};
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
use pathfinder_metal::MetalDevice;

// Types

// External: `font-kit`
pub type FKHandleRef = *mut Handle;

// `canvas`
pub type PFCanvasRef = *mut CanvasRenderingContext2D;
pub type PFPathRef = *mut Path2D;
pub type PFCanvasFontContextRef = *mut CanvasFontContext;
pub type PFFillStyleRef = *mut FillStyle;
pub type PFLineCap = u8;
pub type PFLineJoin = u8;
pub type PFArcDirection = u8;
pub type PFTextAlign = u8;
#[repr(C)]
pub struct PFTextMetrics {
    pub width: f32,
}

// `content`
#[repr(C)]
pub struct PFColorF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
pub struct PFColorU {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// `geometry`
#[repr(C)]
pub struct PFVector2F {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
pub struct PFVector2I {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
pub struct PFRectF {
    pub origin: PFVector2F,
    pub lower_right: PFVector2F,
}
#[repr(C)]
pub struct PFRectI {
    pub origin: PFVector2I,
    pub lower_right: PFVector2I,
}
/// Row-major order.
#[repr(C)]
pub struct PFMatrix2x2F {
    pub m00: f32, pub m01: f32,
    pub m10: f32, pub m11: f32,
}
/// Row-major order.
#[repr(C)]
pub struct PFTransform2F {
    pub matrix: PFMatrix2x2F,
    pub vector: PFVector2F,
}
/// Row-major order.
#[repr(C)]
pub struct PFTransform4F {
    pub m00: f32, pub m01: f32, pub m02: f32, pub m03: f32,
    pub m10: f32, pub m11: f32, pub m12: f32, pub m13: f32,
    pub m20: f32, pub m21: f32, pub m22: f32, pub m23: f32,
    pub m30: f32, pub m31: f32, pub m32: f32, pub m33: f32,
}
#[repr(C)]
pub struct PFPerspective {
    pub transform: PFTransform4F,
    pub window_size: PFVector2I,
}

// `gl`
pub type PFGLDeviceRef = *mut GLDevice;
pub type PFGLVersion = u8;
pub type PFGLFunctionLoader = extern "C" fn(name: *const c_char, userdata: *mut c_void)
                                            -> *const c_void;
// `gpu`
pub type PFGLDestFramebufferRef = *mut DestFramebuffer<GLDevice>;
pub type PFGLRendererRef = *mut Renderer<GLDevice>;

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
pub type PFMetalDestFramebufferRef = *mut DestFramebuffer<MetalDevice>;
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
pub type PFMetalRendererRef = *mut Renderer<MetalDevice>;

// FIXME(pcwalton): Double-boxing is unfortunate. Remove this when `std::raw::TraitObject` is
// stable?
pub type PFResourceLoaderRef = *mut ResourceLoaderWrapper;
pub struct ResourceLoaderWrapper(Box<dyn ResourceLoader>);

// `metal`
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
pub type PFMetalDeviceRef = *mut MetalDevice;

// `renderer`
pub type PFSceneRef = *mut Scene;
pub type PFSceneProxyRef = *mut SceneProxy;
#[repr(C)]
pub struct PFRendererMode {
    pub level: PFRendererLevel,
}
pub type PFDestFramebufferRef = *mut c_void;
#[repr(C)]
pub struct PFRendererOptions {
    pub dest: PFDestFramebufferRef,
    pub background_color: PFColorF,
    pub flags: PFRendererOptionsFlags,
}
pub type PFRendererOptionsFlags = u8;
pub type PFBuildOptionsRef = *mut BuildOptions;
pub type PFRenderTransformRef = *mut RenderTransform;
pub type PFRendererLevel = u8;

// `svg`
pub type PFSVGSceneRef = *mut SVGScene;

// `svg`

/// Returns `NULL` on failure.
#[no_mangle]
pub unsafe extern "C" fn PFSVGSceneCreateWithMemory(bytes: *const c_char, byte_len: usize)
                                                    -> PFSVGSceneRef {
    let data = slice::from_raw_parts(bytes as *const _, byte_len);
    let tree = match Tree::from_data(data, &Options::default()) {
        Ok(tree) => tree,
        Err(_) => return ptr::null_mut(),
    };
    let svg_scene = SVGScene::from_tree(&tree);
    Box::into_raw(Box::new(svg_scene))
}

/// Returns `NULL` on failure.
#[no_mangle]
pub unsafe extern "C" fn PFSVGSceneCreateWithPath(path: *const c_char) -> PFSVGSceneRef {
    let string = to_rust_string(&path, 0);
    let path = PathBuf::from(string);
    let tree = match Tree::from_file(path, &Options::default()) {
        Ok(tree) => tree,
        Err(_) => return ptr::null_mut(),
    };
    let svg_scene = SVGScene::from_tree(&tree);
    Box::into_raw(Box::new(svg_scene))
}

/// Destroys the SVG and returns the scene.
#[no_mangle]
pub unsafe extern "C" fn PFSVGSceneIntoScene(svg: PFSVGSceneRef) -> PFSceneRef {
    Box::into_raw(Box::new((*Box::from_raw(svg)).scene))
}

// Helpers for `canvas`
unsafe fn to_rust_string(ptr: &*const c_char, mut len: usize) -> &str {
    if len == 0 {
        len = libc::strlen(*ptr);
    }
    str::from_utf8(slice::from_raw_parts(*ptr as *const u8, len)).unwrap()
}

// Helpers for `content`

impl PFColorF {
    #[inline]
    pub fn to_rust(&self) -> ColorF {
        ColorF(F32x4::new(self.r, self.g, self.b, self.a))
    }
}

impl PFColorU {
    #[inline]
    pub fn to_rust(&self) -> ColorU {
        ColorU { r: self.r, g: self.g, b: self.b, a: self.a }
    }
}

// Helpers for `geometry`

impl PFRectF {
    #[inline]
    pub fn to_rust(&self) -> RectF {
        RectF::from_points(self.origin.to_rust(), self.lower_right.to_rust())
    }
}

impl PFRectI {
    #[inline]
    pub fn to_rust(&self) -> RectI {
        RectI::from_points(self.origin.to_rust(), self.lower_right.to_rust())
    }
}

impl PFVector2F {
    #[inline]
    pub fn to_rust(&self) -> Vector2F {
        Vector2F::new(self.x, self.y)
    }
}

impl PFVector2I {
    #[inline]
    pub fn to_rust(&self) -> Vector2I {
        Vector2I::new(self.x, self.y)
    }
}

impl PFMatrix2x2F {
    #[inline]
    pub fn to_rust(&self) -> Matrix2x2F {
        Matrix2x2F::row_major(self.m00, self.m01, self.m10, self.m11)
    }
}

impl PFTransform2F {
    #[inline]
    pub fn to_rust(&self) -> Transform2F {
        Transform2F { matrix: self.matrix.to_rust(), vector: self.vector.to_rust() }
    }
}

impl PFTransform4F {
    #[inline]
    pub fn to_rust(&self) -> Transform4F {
        Transform4F::row_major(self.m00, self.m01, self.m02, self.m03,
                                self.m10, self.m11, self.m12, self.m13,
                                self.m20, self.m21, self.m22, self.m23,
                                self.m30, self.m31, self.m32, self.m33)
    }
}

impl PFPerspective {
    #[inline]
    pub fn to_rust(&self) -> Perspective {
        Perspective {
            transform: self.transform.to_rust(),
            window_size: self.window_size.to_rust(),
        }
    }
}

