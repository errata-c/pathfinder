
use pathfinder_canvas::{FillStyle};

use crate::{
    PFColorU, PFFillStyleRef,
};

#[no_mangle]
pub unsafe extern "C" fn PFFillStyleCreateColor(color: *const PFColorU) -> PFFillStyleRef {
    Box::into_raw(Box::new(FillStyle::Color((*color).to_rust())))
}

/*
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
*/

#[no_mangle]
pub unsafe extern "C" fn PFFillStyleDestroy(fill_style: PFFillStyleRef) {
    drop(Box::from_raw(fill_style))
}
