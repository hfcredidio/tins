extern "C" {
    fn cdfchi_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdfpoi_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdfchn_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdff_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdft_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdfgam_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdfnor_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdftnc_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdfbet_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, d: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdffnc_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, d: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdfnbn_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, d: &mut f64, status: &mut i32, bound: &mut f64);
    fn cdfbin_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, d: &mut f64, status: &mut i32, bound: &mut f64);
}

fn get_result(
    name: &str,
    status: i32,
    bound: f64,
    result: f64,
    return_bound: bool,
) -> Result<f64, String> {
    match status {
        _ if status < 0 => Err(format!(
            "{}, (Fortran) input parameter {} is out of range",
            name, -status
        )),
        0 => Ok(result),
        1 => if return_bound {
            Ok(bound)
        } else {
            Err(format!(
                "Answer appears to be lower than lowest search bound ({})",
                bound
            ))
        },
        2 => if return_bound {
            Ok(bound)
        } else {
            Err(format!(
                "Answer appears to be higher than highest search bound ({})",
                bound
            ))
        },
        3 | 4 => Err(format!("Two parameters that should sum to 1.0 do not")),
        10 => Err(format!("Computational error")),
        _ => Err(format!("Unknown error")),
    }
}


macro_rules! call_cdflib_safe {
    ($func:ident, $retval:expr, $return_bound:expr, $which:ident, $status:ident, $bound:ident, $( $arg:ident ),*) => {
        if $bound.is_nan() | $( $arg.is_nan() )|* {
            Err(format!("tins::{} NaN passed as an argument", stringify!($func)))
        } else {
            unsafe {
                $func(&mut $which, $( &mut $arg, )* &mut $status, &mut $bound);
            }
            let result = [$( $arg ),*][$retval];
            get_result(stringify!($func), $status, $bound, result, $return_bound)
        }
    };
}


pub fn cdfbet3_wrap(p: f64, b: f64, x: f64) -> f64 {
    let mut p = p;
    let mut b = b;
    let mut x = x;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut y = 1.0 - x;
    let mut a: f64 = 0.0;
    let mut bound: f64 = 00.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfbet_, 4, true, which, status, bound, p, q, x, y, a, b) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfbet4_wrap(a: f64, p: f64, x: f64) -> f64 {
    let mut a = a;
    let mut p = p;
    let mut x = x;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut y = 1.0 - x;
    let mut b: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfbet_, 5, true, which, status, bound, p, q, x, y, a, b) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfbin2_wrap(p: f64, xn: f64, pr: f64) -> f64 {
    let mut p = p;
    let mut xn = xn;
    let mut pr = pr;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut s: f64 = 0.0;
    let mut ompr = 1.0 - pr;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfbin_, 2, true, which, status, bound, p, q, s, xn, pr, ompr) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfbin3_wrap(s: f64, p: f64, pr: f64) -> f64 {
    let mut s = s;
    let mut p = p;
    let mut pr = pr;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut xn: f64 = 0.0;
    let mut ompr = 1.0 - pr;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfbin_, 3, true, which, status, bound, p, q, s, xn, pr, ompr) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfchi3_wrap(p: f64, x: f64) -> f64 {
    let mut p = p;
    let mut x = x;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut df: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfchi_, 2, true, which, status, bound, p, q, x, df) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfchn1_wrap(x: f64, df: f64, nc: f64) -> f64 {
    let mut x = x;
    let mut df = df;
    let mut nc = nc;
    let mut which = 1;
    let mut q: f64 = 0.0;
    let mut p: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfchn_, 0, true, which, status, bound, p, q, x, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfchn2_wrap(p: f64, df: f64, nc: f64) -> f64 {
    let mut p = p;
    let mut df = df;
    let mut nc = nc;
    let mut which = 2;
    let mut q: f64 = 1.0 - p;
    let mut x: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfchn_, 2, false, which, status, bound, p, q, x, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfchn3_wrap(x: f64, p: f64, nc: f64) -> f64 {
    let mut x = x;
    let mut p = p;
    let mut nc = nc;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut df: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfchn_, 3, true, which, status, bound, p, q, x, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfchn4_wrap(x: f64, df: f64, p: f64) -> f64 {
    let mut x = x;
    let mut df = df;
    let mut p = p;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut nc: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfchn_, 4, true, which, status, bound, p, q, x, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdff3_wrap(p: f64, dfd: f64, f: f64) -> f64 {
    let mut p = p;
    let mut dfd = dfd;
    let mut f = f;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut dfn: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdff_, 2, true, which, status, bound, p, q, f, dfn, dfd) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdff4_wrap(dfn: f64, p: f64, f: f64) -> f64 {
    let mut dfn = dfn;
    let mut p = p;
    let mut f = f;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut dfd: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdff_, 4, true, which, status, bound, p, q, f, dfn, dfd) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdffnc1_wrap(dfn: f64, dfd: f64, nc: f64, f: f64) -> f64 {
    let mut dfn = dfn;
    let mut dfd = dfd;
    let mut nc = nc;
    let mut f = f;
    let mut which = 1;
    let mut q: f64 = 0.0;
    let mut p: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdffnc_, 0, false, which, status, bound, p, q, f, dfn, dfd, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdffnc2_wrap(dfn: f64, dfd: f64, nc: f64, p: f64) -> f64 {
    let mut dfn = dfn;
    let mut dfd = dfd;
    let mut nc = nc;
    let mut p = p;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut f: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdffnc_, 2, true, which, status, bound, p, q, f, dfn, dfd, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdffnc3_wrap(p: f64, dfd: f64, nc: f64, f: f64) -> f64 {
    let mut p = p;
    let mut dfd = dfd;
    let mut nc = nc;
    let mut f = f;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut dfn: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdffnc_, 3, true, which, status, bound, p, q, f, dfn, dfd, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdffnc4_wrap(dfn: f64, p: f64, nc: f64, f: f64) -> f64 {
    let mut dfn = dfn;
    let mut p = p;
    let mut nc = nc;
    let mut f = f;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut dfd: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdffnc_, 4, true, which, status, bound, p, q, f, dfn, dfd, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdffnc5_wrap(dfn: f64, dfd: f64, p: f64, f: f64) -> f64 {
    let mut dfn = dfn;
    let mut dfd = dfd;
    let mut p = p;
    let mut f = f;
    let mut which = 5;
    let mut q = 1.0 - p;
    let mut nc: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdffnc_, 5, true, which, status, bound, p, q, f, dfn, dfd, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfgam1_wrap(scl: f64, shp: f64, x: f64) -> f64 {
    let mut scl = scl;
    let mut shp = shp;
    let mut x = x;
    let mut which = 1;
    let mut q: f64 = 0.0;
    let mut p: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfgam_, 0, false, which, status, bound, p, q, x, shp, scl) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfgam2_wrap(scl: f64, shp: f64, p: f64) -> f64 {
    let mut scl = scl;
    let mut shp = shp;
    let mut p = p;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut x: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfgam_, 2, true, which, status, bound, p, q, x, shp, scl) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfgam3_wrap(scl: f64, p: f64, x: f64) -> f64 {
    let mut scl = scl;
    let mut p = p;
    let mut x = x;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut shp: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfgam_, 3, true, which, status, bound, p, q, x, shp, scl) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfgam4_wrap(p: f64, shp: f64, x: f64) -> f64 {
    let mut p = p;
    let mut shp = shp;
    let mut x = x;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut scl: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfgam_, 4, true, which, status, bound, p, q, x, shp, scl) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfnbn2_wrap(p: f64, xn: f64, pr: f64) -> f64 {
    let mut p = p;
    let mut xn = xn;
    let mut pr = pr;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut s: f64 = 0.0;
    let mut ompr = 1.0 - pr;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfnbn_, 2, true, which, status, bound, p, q, s, xn, pr, ompr) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfnbn3_wrap(s: f64, p: f64, pr: f64) -> f64 {
    let mut s = s;
    let mut p = p;
    let mut pr = pr;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut xn: f64 = 0.0;
    let mut ompr = 1.0 - pr;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfnbn_, 3, true, which, status, bound, p, q, s, xn, pr, ompr) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfnor3_wrap(p: f64, std: f64, x: f64) -> f64 {
    let mut p = p;
    let mut std = std;
    let mut x = x;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut mn: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfnor_, 3, true, which, status, bound, p, q, x, mn, std) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfnor4_wrap(mn: f64, p: f64, x: f64) -> f64 {
    let mut mn = mn;
    let mut p = p;
    let mut x = x;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut std: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfnor_, 4, true, which, status, bound, p, q, x, mn, std) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdfpoi2_wrap(p: f64, xlam: f64) -> f64 {
    let mut p = p;
    let mut xlam = xlam;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut s: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdfpoi_, 2, true, which, status, bound, p, q, s, xlam) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdft1_wrap(df: f64, t: f64) -> f64 {
    let mut df = df;
    let mut t = t;
    let mut which = 1;
    let mut q: f64 = 0.0;
    let mut p: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdft_, 0, false, which, status, bound, p, q, t, df) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdft2_wrap(df: f64, p: f64) -> f64 {
    let mut df = df;
    let mut p = p;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut t: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdft_, 2, true, which, status, bound, p, q, t, df) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdft3_wrap(p: f64, t: f64) -> f64 {
    let mut p = p;
    let mut t = t;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut df: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdft_, 3, true, which, status, bound, p, q, t, df) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdftnc1_wrap(df: f64, nc: f64, t: f64) -> f64 {
    let mut df = df;
    let mut nc = nc;
    let mut t = t;
    let mut which = 1;
    let mut q: f64 = 0.0;
    let mut p: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdftnc_, 0, true, which, status, bound, p, q, t, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdftnc2_wrap(df: f64, nc: f64, p: f64) -> f64 {
    let mut df = df;
    let mut nc = nc;
    let mut p = p;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut t: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdftnc_, 2, true, which, status, bound, p, q, t, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdftnc3_wrap(p: f64, nc: f64, t: f64) -> f64 {
    let mut p = p;
    let mut nc = nc;
    let mut t = t;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut df: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdftnc_, 3, true, which, status, bound, p, q, t, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn cdftnc4_wrap(df: f64, p: f64, t: f64) -> f64 {
    let mut df = df;
    let mut p = p;
    let mut t = t;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut nc: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(cdftnc_, 4, true, which, status, bound, p, q, t, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}
fn main() {
}
