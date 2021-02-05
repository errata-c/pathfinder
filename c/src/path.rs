use crate::{
    PFPathRef, PFVector2F, PFArcDirection, PFRectF
};

use pathfinder_content::outline::ArcDirection;
use pathfinder_canvas::Path2D;

pub const PF_ARC_DIRECTION_CW:  u8 = 0;
pub const PF_ARC_DIRECTION_CCW: u8 = 1;


#[no_mangle]
pub unsafe extern "C" fn PFPathCreate() -> PFPathRef {
    Box::into_raw(Box::new(Path2D::new()))
}

#[no_mangle]
pub unsafe extern "C" fn PFPathDestroy(path: PFPathRef) {
    drop(Box::from_raw(path))
}

#[no_mangle]
pub unsafe extern "C" fn PFPathClone(path: PFPathRef) -> PFPathRef {
    Box::into_raw(Box::new((*path).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn PFPathMoveTo(path: PFPathRef, to: *const PFVector2F) {
    (*path).move_to((*to).to_rust())
}

#[no_mangle]
pub unsafe extern "C" fn PFPathLineTo(path: PFPathRef, to: *const PFVector2F) {
    (*path).line_to((*to).to_rust())
}

#[no_mangle]
pub unsafe extern "C" fn PFPathQuadraticCurveTo(path: PFPathRef,
                                                ctrl: *const PFVector2F,
                                                to: *const PFVector2F) {
    (*path).quadratic_curve_to((*ctrl).to_rust(), (*to).to_rust())
}

#[no_mangle]
pub unsafe extern "C" fn PFPathBezierCurveTo(path: PFPathRef,
                                             ctrl0: *const PFVector2F,
                                             ctrl1: *const PFVector2F,
                                             to: *const PFVector2F) {
    (*path).bezier_curve_to((*ctrl0).to_rust(), (*ctrl1).to_rust(), (*to).to_rust())
}

#[no_mangle]
pub unsafe extern "C" fn PFPathArc(path: PFPathRef,
                                   center: *const PFVector2F,
                                   radius: f32,
                                   start_angle: f32,
                                   end_angle: f32,
                                   direction: PFArcDirection) {
    let direction = match direction {
        PF_ARC_DIRECTION_CW  => ArcDirection::CW,
        PF_ARC_DIRECTION_CCW => ArcDirection::CCW,
        _                    => panic!("Invalid Pathfinder arc direction!"),
    };
    (*path).arc((*center).to_rust(), radius, start_angle, end_angle, direction)
}

#[no_mangle]
pub unsafe extern "C" fn PFPathArcTo(path: PFPathRef,
                                     ctrl: *const PFVector2F,
                                     to: *const PFVector2F,
                                     radius: f32) {
    (*path).arc_to((*ctrl).to_rust(), (*to).to_rust(), radius)
}

#[no_mangle]
pub unsafe extern "C" fn PFPathRect(path: PFPathRef, rect: *const PFRectF) {
    (*path).rect((*rect).to_rust())
}

#[no_mangle]
pub unsafe extern "C" fn PFPathEllipse(path: PFPathRef,
                                       center: *const PFVector2F,
                                       axes: *const PFVector2F,
                                       rotation: f32,
                                       start_angle: f32,
                                       end_angle: f32) {
    (*path).ellipse((*center).to_rust(), (*axes).to_rust(), rotation, start_angle, end_angle)
}

#[no_mangle]
pub unsafe extern "C" fn PFPathClosePath(path: PFPathRef) {
    (*path).close_path()
}