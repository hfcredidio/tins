use crate::complex::Complex;

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
        ierr: &mut i32,
    );
}

fn get_result<T: Complex>(name: &str, ierr: i32, result: T) -> Result<T, String> {
    match ierr {
        0 => Ok(result),
        -1 => Err(format!("amos.{}: Underflow encountered", name)),
        1 => Err(format!("amos.{}: Input error", name)),
        2 => Err(format!("amos.{}: Overflow encountered", name)),
        3 => Err(format!("amos.{}: Computation completed losses of signifcance by argument reduction produce less than half of machine accuracy", name)),
        4 => Err(format!("amos.{}: No computation complete loss of accuracy by argument reduction", name)),
        5 => Err(format!("amos.{}: No computation, algorithm termination condition not met", name)),
        _ => Err(format!("amos.{}: Unknown error", name)),
    }
}

macro_rules! _amos {
    (@call zairy_, $ierr:ident, $($arg:ident),*) => {
        let mut nz = 0;
        unsafe { zairy_($(&mut $arg),*, &mut nz, &mut $ierr); }
        $ierr = if nz == 1 { -nz } else { $ierr };
    };
    (@call $func:ident, $ierr:ident, $($arg:ident),*) => {
        unsafe { $func($(&mut $arg),*, &mut $ierr); }
    };
    (@wrap $wrap:ident, $func:ident, $id:expr, $kode:expr) => {
        pub fn $wrap<T: Complex + Copy>(z: T) -> T {
            let mut zr = z.real();
            let mut zi = z.imag();
            let mut id = $id;
            let mut kode = $kode;

            let mut air = 0.0;
            let mut aii = 0.0;
            let mut ierr = 0;
            _amos!(@call $func, ierr, zr, zi, id, kode, air, aii);
            let res = T::from_tuple((air, aii));
            match get_result(stringify!($func), ierr, res) {
                Err(msg) => { eprintln!("{}", msg); res }
                Ok(res) => res,
            }
        }
    };
}

_amos!(@wrap airy_a, zairy_, 0, 1);
_amos!(@wrap airy_b, zbiry_, 0, 1);
_amos!(@wrap es_airy_a, zairy_, 0, 2);
_amos!(@wrap es_airy_b, zbiry_, 0, 2);

_amos!(@wrap der_airy_a, zairy_, 1, 1);
_amos!(@wrap der_airy_b, zbiry_, 1, 1);
_amos!(@wrap der_es_airy_a, zairy_, 1, 2);
_amos!(@wrap der_es_airy_b, zbiry_, 1, 2);
