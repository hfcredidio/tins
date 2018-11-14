use complex::Complex;
use std::collections::HashMap;

extern "C" {
    fn zairy_(
        zr: &mut f64,
        zi: &mut f64,
        id: &mut i32,
        kode: &mut i32,
        air: &mut f64,
        aii: &mut f64,
        nz: &mut i32,
        ierr: &mut i32,
    );
    fn zbiry_(
        zr: &mut f64,
        zi: &mut f64,
        id: &mut i32,
        kode: &mut i32,
        air: &mut f64,
        aii: &mut f64,
        nz: &mut i32,
        ierr: &mut i32,
    );
}

macro_rules! error_messages {
    ($func:ident, 5) => {
        "No computation, algorithm termination condition not met"
    };
    ($func:ident, 4) => {
        "No computation complete loss of accuracy by argument reduction"
    };
    ($func:ident, 3) => {{
        "Computation completed losses of signifcance by argument reduction produce less than half of machine accuracy"
    }};
    ($func:ident, 2) => { "Overflow" };
    ($func:ident, 1) => { "Input Error" };
    ($func:ident, $ierr:expr) => { "Unknown Error" };
}

macro_rules! get_result {
    ($func:ident, $ierr:expr, $result:expr) => {
        match $ierr {
            0 => Ok($result),
            1 => Err(error_messages!($func, 1)),
            2 => Err(error_messages!($func, 2)),
            3 => Err(error_messages!($func, 3)),
            4 => Err(error_messages!($func, 4)),
            5 => Err(error_messages!($func, 5)),
            _ => Err(error_messages!($func, 6)),
        }
    };
}

macro_rules! _amos {
    (@wrap $wrap:ident, $func:ident, $id:expr, $kode:expr) => {
        pub fn $wrap<T: Complex>(z: T) -> T {
            let mut zr = z.real();
            let mut zi = z.imag();
            let mut id = $id;
            let mut kode = $kode;

            let mut air = 0.0;
            let mut aii = 0.0;
            let mut ierr = 0;
            let mut nz = 0;
            unsafe {
                $func(
                    &mut zr, &mut zi, &mut id, &mut kode, &mut air, &mut aii, &mut nz, &mut ierr,
                );
            }
            let res = T::from_tuple((air, aii));
            match get_result!($func, ierr, res) {
                Err(msg) => {
                    eprintln!("{}", msg);
                    T::from_tuple((0.0, 0.0))
                }
                Ok(res) => res,
            }
        }
    };
}

_amos!(@wrap airy, zairy_, 0, 1);
_amos!(@wrap biry, zbiry_, 0, 1);
_amos!(@wrap airye, zairy_, 0, 2);
_amos!(@wrap birye, zbiry_, 0, 2);

_amos!(@wrap dairy, zairy_, 1, 1);
_amos!(@wrap dbiry, zbiry_, 1, 1);
_amos!(@wrap dairye, zairy_, 1, 2);
_amos!(@wrap dbirye, zbiry_, 1, 2);
