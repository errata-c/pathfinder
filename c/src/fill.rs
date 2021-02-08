
use pathfinder_canvas::{FillStyle};
use pathfinder_content::gradient::Gradient;
use pathfinder_content::pattern::{Image, Pattern};
use pathfinder_geometry::line_segment::LineSegment2F;
use pathfinder_simd::x86::F32x2;

use crate::{
    PFColorU, PFFillStyleRef, PFVector2F
};

pub const PF_FILLSTYLE_COLOR:       u8 = 0;
pub const PF_FILLSTYLE_GRADIENT:    u8 = 1;
pub const PF_FILLSTYLE_PATTERN:     u8 = 2;

#[no_mangle]
pub unsafe extern "C" fn PFFillStyleCreateColor(color: *const PFColorU) -> PFFillStyleRef {
    Box::into_raw(Box::new(FillStyle::Color((*color).to_rust())))
}

#[no_mangle]
pub unsafe extern "C" fn PFFillStyleClone(fill: PFFillStyleRef) -> PFFillStyleRef {
    Box::into_raw(Box::new((*fill).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn PFFillStyleGetKind(fill: PFFillStyleRef) -> u8 {
    match *fill {
        FillStyle::Gradient(_) => PF_FILLSTYLE_GRADIENT,
        FillStyle::Color(_) => PF_FILLSTYLE_COLOR,
        FillStyle::Pattern(_) => PF_FILLSTYLE_PATTERN,
    }
}

#[no_mangle]
pub unsafe extern "C" fn PFFillStyleCreateGradientLinear(from: *const PFVector2F, to: *const PFVector2F) -> PFFillStyleRef {
    Box::into_raw(Box::new(FillStyle::Gradient(
        Gradient::linear_from_points((*from).to_rust(), (*to).to_rust())
    )))
}

#[no_mangle]
pub unsafe extern "C" fn PFFillStyleCreateGradientRadial(from: *const PFVector2F, to: *const PFVector2F, radii: *const PFVector2F) -> PFFillStyleRef {
    Box::into_raw(Box::new(FillStyle::Gradient(
        Gradient::radial(
            LineSegment2F::new((*from).to_rust(), (*to).to_rust()),
            F32x2::new((*radii).x, (*radii).y)
            )
    )))
}

#[no_mangle]
pub unsafe extern "C" fn PFFillStyleGradientAddStop(gradient: PFFillStyleRef, color: PFColorU, offset: f32) {
    match &mut(*gradient) {
        FillStyle::Gradient(grad) => grad.add_color_stop(color.to_rust(), offset),
        _ => panic!("The PFFillStyleRef passed into PFFillStyleGradientAddStop is not a gradient!"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn PFFillStyleDestroy(fill_style: PFFillStyleRef) {
    drop(Box::from_raw(fill_style))
}
