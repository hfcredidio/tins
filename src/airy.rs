use crate::complex::Complex;
use crate::amos;
use crate::cephes;


macro_rules! any_is_inf {
    ($($x:ident),*) => {
        $(!$x.real().is_finite() || !$x.imag().is_finite())||*
    };
}


pub fn airy_a<T: Complex + Copy>(x: T) -> T {
    if any_is_inf!(x) {
        T::from_tuple((std::f64::NAN, std::f64::NAN))
    } else if x.imag() == 0.0 && (-10.0 < x.real() || x.real() < 10.0) {
        T::from_tuple((cephes::airy(x.real()).0, 0.0))
    } else {
        amos::airy_a(x)
    }
}
