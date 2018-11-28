use crate::complex::Complex;

pub fn airy_a<T: Complex>(z: T) -> T {
    if z.real().is_nan() | z.imag().is_nan() | z.real().is_infinite() | z.imag().is_infinite() {
        T::from_tuple((std::f64::NAN, std::f64::NAN))
    } else if (z.imag() == 0.0) & (z.real().abs() < 10.0) {
        T::from_tuple((crate::cephes::airy(z.real()).0, 0.0))
    } else {
        crate::amos::airy_a(z)
    }
}


macro_rules! assert_approx_eq_or_nan {
    ($x:expr, $y:expr, $tol:expr) => {
        if ($x).is_nan() & ($y).is_nan() {
            assert!(true)
        } else {
            assert_approx_eq!($x, $y, $tol)
        }
    };
}


#[cfg(test)]
mod tests {
    use crate::npy::NpyData;
    use crate::complex::{c64, Complex};
    use crate::std::io::Read;

    #[test]
    fn f64_airy_a() {
        let mut buf = vec![];
        std::fs::File::open("/home/heitor/rust/tins/src/test_data/f64_airy_a.npy")
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        let data: Vec<f64> = NpyData::from_bytes(&buf).unwrap().to_vec();
        for xy in data.chunks(2) {
            let x = xy[0];
            let y = xy[1];
            assert_approx_eq_or_nan!(y, crate::special::airy_a(x), 1e-10);
        }
    }

    #[test]
    fn c64_airy_a() {
        let mut buf = vec![];
        std::fs::File::open("/home/heitor/rust/tins/src/test_data/c64_airy_a.npy")
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        let data: Vec<f64> = NpyData::from_bytes(&buf).unwrap().to_vec();
        let mut i = 0;
        loop {
            let x_re = data[i];
            let x_im = data[i + 1];
            let y_re = data[data.len() / 2 + i];
            let y_im = data[data.len() / 2 + i + 1];
            let z = crate::special::airy_a(c64::from_tuple((x_re, x_im)));
            //println!("{:?} {:?} {:?} {:?}", x_re, x_im, y_re, y_im);
            assert_approx_eq_or_nan!(y_re, z.real(), 1e-10);
            assert_approx_eq_or_nan!(y_im, z.imag(), 1e-10);
            i += 2;
            if i >= data.len() / 4 { break }
        }
    }
}
