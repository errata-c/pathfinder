
#![allow(non_snake_case)]

use std::slice;
use std::str;
use std::os::raw::{c_char};

use pathfinder_content::stroke::LineCap;
use pathfinder_canvas::{Canvas, CanvasFontContext, FillStyle, LineJoin};
use pathfinder_canvas::{TextAlign};
use pathfinder_content::fill::FillRule;
use pathfinder_geometry::vector::{Vector2F};
use pathfinder_canvas::TextMetrics;

use crate::{
    PFVector2F, PFCanvasRef, PFCanvasFontContextRef, FKHandleRef, PFSceneRef, PFRectF, PFPathRef,
    PFFillStyleRef, PFColorU, PFTransform2F, PFTextAlign, PFMatrix2x2F, PFLineJoin, PFTextMetrics,
    PFLineCap,
};

// `canvas`

pub const PF_LINE_CAP_BUTT:     u8 = 0;
pub const PF_LINE_CAP_SQUARE:   u8 = 1;
pub const PF_LINE_CAP_ROUND:    u8 = 2;

pub const PF_LINE_JOIN_MITER:   u8 = 0;
pub const PF_LINE_JOIN_BEVEL:   u8 = 1;
pub const PF_LINE_JOIN_ROUND:   u8 = 2;

pub const PF_TEXT_ALIGN_LEFT:   u8 = 0;
pub const PF_TEXT_ALIGN_CENTER: u8 = 1;
pub const PF_TEXT_ALIGN_RIGHT:  u8 = 2;


// `canvas`

/// This function internally adds a reference to the font context. Therefore, if you created the
/// font context, you must release it yourself to avoid a leak.
#[no_mangle]
pub unsafe extern "C" fn PFCanvasCreate(font_context: PFCanvasFontContextRef,
                                        size: *const PFVector2F)
                                        -> PFCanvasRef {
    Box::into_raw(Box::new(Canvas::new((*size).to_rust()).get_context_2d((*font_context).clone())))
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasDestroy(canvas: PFCanvasRef) {
    drop(Box::from_raw(canvas))
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasFontContextCreateWithSystemSource() -> PFCanvasFontContextRef {
    Box::into_raw(Box::new(CanvasFontContext::from_system_source()))
}

/// Creates a Pathfinder font context from a set of `font-kit` fonts.
///
/// Note that `font-kit` itself has a C API. You can use this to load fonts from memory with e.g.
/// `FKHandleCreateWithMemory()`.
#[no_mangle]
pub unsafe extern "C" fn PFCanvasFontContextCreateWithFonts(fonts: *const FKHandleRef,
                                                            font_count: usize)
                                                            -> PFCanvasFontContextRef {
    let fonts = slice::from_raw_parts(fonts, font_count);
    Box::into_raw(Box::new(CanvasFontContext::from_fonts(fonts.into_iter().map(|font| {
        (**font).clone()
    }))))
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasFontContextPrintFonts(font_context: PFCanvasFontContextRef) {
    let font_source = (*Box::from_raw(font_context)).font_source();
    let handles = font_source.all_fonts().expect("Failed to select all fonts");
    for handle in handles {
        let font = handle.load().expect("Failed to load font from handle");
        let ps_name = font.postscript_name().expect("Failed to get postscript name");

        println!("{:<32}, {:<32}", font.full_name(), ps_name);
    }
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasFontContextAddRef(font_context: PFCanvasFontContextRef)
                                                   -> PFCanvasFontContextRef {
    Box::into_raw(Box::new((*font_context).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasFontContextRelease(font_context: PFCanvasFontContextRef) {
    drop(Box::from_raw(font_context))
}

/// This function takes ownership of the supplied canvas and will automatically destroy it when
/// the scene is destroyed.
#[no_mangle]
pub unsafe extern "C" fn PFCanvasCreateScene(canvas: PFCanvasRef) -> PFSceneRef {
    Box::into_raw(Box::new(Box::from_raw(canvas).into_canvas().into_scene()))
}

// Drawing rectangles

#[no_mangle]
pub unsafe extern "C" fn PFCanvasFillRect(canvas: PFCanvasRef, rect: *const PFRectF) {
    (*canvas).fill_rect((*rect).to_rust())
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasStrokeRect(canvas: PFCanvasRef, rect: *const PFRectF) {
    (*canvas).stroke_rect((*rect).to_rust())
}

// Drawing text

#[no_mangle]
pub unsafe extern "C" fn PFCanvasFillText(canvas: PFCanvasRef,
                                          string: *const c_char,
                                          string_len: usize,
                                          position: *const PFVector2F) {
    (*canvas).fill_text(to_rust_string(&string, string_len), (*position).to_rust())
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasStrokeText(canvas: PFCanvasRef,
                                            string: *const c_char,
                                            string_len: usize,
                                            position: *const PFVector2F) {
    (*canvas).stroke_text(to_rust_string(&string, string_len), (*position).to_rust())
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasMeasureText(canvas: PFCanvasRef,
                                             string: *const c_char,
                                             string_len: usize,
                                             out_text_metrics: *mut PFTextMetrics) {
    debug_assert!(!out_text_metrics.is_null());
    *out_text_metrics = (*canvas).measure_text(to_rust_string(&string, string_len)).to_c()
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetLineWidth(canvas: PFCanvasRef, new_line_width: f32) {
    (*canvas).set_line_width(new_line_width)
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetLineCap(canvas: PFCanvasRef, new_line_cap: PFLineCap) {
    (*canvas).set_line_cap(match new_line_cap {
        PF_LINE_CAP_BUTT   => LineCap::Butt,
        PF_LINE_CAP_SQUARE => LineCap::Square,
        PF_LINE_CAP_ROUND  => LineCap::Round,
        _                  => panic!("Invalid Pathfinder line cap style!"),
    });
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetLineJoin(canvas: PFCanvasRef, new_line_join: PFLineJoin) {
    (*canvas).set_line_join(match new_line_join {
        PF_LINE_JOIN_MITER => LineJoin::Miter,
        PF_LINE_JOIN_BEVEL => LineJoin::Bevel,
        PF_LINE_JOIN_ROUND => LineJoin::Round,
        _                  => panic!("Invalid Pathfinder line join style!"),
    });
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetMiterLimit(canvas: PFCanvasRef, new_miter_limit: f32) {
    (*canvas).set_miter_limit(new_miter_limit);
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetLineDash(canvas: PFCanvasRef,
                                             new_line_dashes: *const f32,
                                             new_line_dash_count: usize) {
    (*canvas).set_line_dash(slice::from_raw_parts(new_line_dashes, new_line_dash_count).to_vec())
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetTransform(canvas: PFCanvasRef,
                                              transform: *const PFTransform2F) {
    (*canvas).set_transform(&(*transform).to_rust());
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasGetTransform(canvas: PFCanvasRef) -> PFTransform2F {
	PFTransform2F{
		matrix: PFMatrix2x2F{
			m00: (*canvas).transform().matrix.m11(),
			m01: (*canvas).transform().matrix.m12(),
			m10: (*canvas).transform().matrix.m21(),
			m11: (*canvas).transform().matrix.m22(),
		},
		vector: PFVector2F{
			x: (*canvas).transform().vector.x(),
			y: (*canvas).transform().vector.y(),
		},
	}	
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasResetTransform(canvas: PFCanvasRef) {
    (*canvas).reset_transform();
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasRotate(canvas: PFCanvasRef, angle: f32) {
    (*canvas).rotate(angle);
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasScale(canvas: PFCanvasRef, x_scale: f32, y_scale: f32) {
    (*canvas).scale(Vector2F::new(x_scale, y_scale));
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasTranslate(canvas: PFCanvasRef, x_offset: f32, y_offset: f32) {
    (*canvas).translate(Vector2F::new(x_offset, y_offset));
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSave(canvas: PFCanvasRef) {
    (*canvas).save();
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasRestore(canvas: PFCanvasRef) {
    (*canvas).restore();
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetLineDashOffset(canvas: PFCanvasRef, new_offset: f32) {
    (*canvas).set_line_dash_offset(new_offset)
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetFontByPostScriptName(canvas: PFCanvasRef,
                                                         postscript_name: *const c_char,
                                                         postscript_name_len: usize) {
    (*canvas).set_font(to_rust_string(&postscript_name, postscript_name_len))
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetFontSize(canvas: PFCanvasRef, new_font_size: f32) {
    (*canvas).set_font_size(new_font_size)
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetTextAlign(canvas: PFCanvasRef, new_text_align: PFTextAlign) {
    (*canvas).set_text_align(match new_text_align {
        PF_TEXT_ALIGN_CENTER => TextAlign::Center,
        PF_TEXT_ALIGN_RIGHT  => TextAlign::Right,
        PF_TEXT_ALIGN_LEFT   => TextAlign::Left,
        _                    => TextAlign::Left,
    });
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetFillStyle(canvas: PFCanvasRef, fill_style: PFFillStyleRef) {
    // FIXME(pcwalton): Avoid the copy?
    (*canvas).set_fill_style((*fill_style).clone())
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetFillColor(canvas: PFCanvasRef, fill_color: PFColorU) {
    (*canvas).set_fill_style(FillStyle::Color(fill_color.to_rust()))
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetStrokeStyle(canvas: PFCanvasRef,
                                                stroke_style: PFFillStyleRef) {
    // FIXME(pcwalton): Avoid the copy?
    (*canvas).set_stroke_style((*stroke_style).clone())
}

#[no_mangle]
pub unsafe extern "C" fn PFCanvasSetStrokeColor(canvas: PFCanvasRef,
                                                stroke_color: PFColorU) {
    (*canvas).set_stroke_style(FillStyle::Color(stroke_color.to_rust()))
}


/// This function automatically destroys the path. If you wish to use the path again, clone it
/// first.
#[no_mangle]
pub unsafe extern "C" fn PFCanvasFillPath(canvas: PFCanvasRef, path: PFPathRef) {
    // TODO(pcwalton): Expose fill rules to the C API.
    (*canvas).fill_path(*Box::from_raw(path), FillRule::Winding)
}

/// This function automatically destroys the path. If you wish to use the path again, clone it
/// first.
#[no_mangle]
pub unsafe extern "C" fn PFCanvasStrokePath(canvas: PFCanvasRef, path: PFPathRef) {
    (*canvas).stroke_path(*Box::from_raw(path))
}

// Helpers
unsafe fn to_rust_string(ptr: &*const c_char, mut len: usize) -> &str {
    if len == 0 {
        len = libc::strlen(*ptr);
    }
    str::from_utf8(slice::from_raw_parts(*ptr as *const u8, len)).unwrap()
}

trait TextMetricsExt {
    fn to_c(&self) -> PFTextMetrics;
}

impl TextMetricsExt for TextMetrics {
    fn to_c(&self) -> PFTextMetrics {
        PFTextMetrics { width: self.width() }
    }
}