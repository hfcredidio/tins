extern "C" {
    #[link_name = "cdfbet_"]
    fn btdtri_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, d: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdfbin_"]
    fn bdtri_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, d: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdfchi_"]
    fn chdtriv_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdfchn_"]
    fn chndtr_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdff_"]
    fn fdtr_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdffnc_"]
    fn ncfdtr_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, d: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdfgam_"]
    fn gdtr_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdfnbn_"]
    fn nbdtr_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, d: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdfnor_"]
    fn nrdtr_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdfpoi_"]
    fn pdtr_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdft_"]
    fn stdtr_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, status: &mut i32, bound: &mut f64);

    #[link_name = "cdftnc_"]
    fn nctdtr_(which: &mut i32, p: &mut f64, q: &mut f64, a: &mut f64, b: &mut f64, c: &mut f64, status: &mut i32, bound: &mut f64);
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


pub fn btdtria(p: f64, b: f64, x: f64) -> f64 {
    let mut p = p;
    let mut b = b;
    let mut x = x;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut y = 1.0 - x;
    let mut a: f64 = 0.0;
    let mut bound: f64 = 00.0;
    let mut status = 10;
    match call_cdflib_safe!(btdtri_, 4, true, which, status, bound, p, q, x, y, a, b) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn btdtrib(a: f64, p: f64, x: f64) -> f64 {
    let mut a = a;
    let mut p = p;
    let mut x = x;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut y = 1.0 - x;
    let mut b: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(btdtri_, 5, true, which, status, bound, p, q, x, y, a, b) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn bdtrik(p: f64, xn: f64, pr: f64) -> f64 {
    let mut p = p;
    let mut xn = xn;
    let mut pr = pr;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut s: f64 = 0.0;
    let mut ompr = 1.0 - pr;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(bdtri_, 2, true, which, status, bound, p, q, s, xn, pr, ompr) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn bdtrin(s: f64, p: f64, pr: f64) -> f64 {
    let mut s = s;
    let mut p = p;
    let mut pr = pr;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut xn: f64 = 0.0;
    let mut ompr = 1.0 - pr;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(bdtri_, 3, true, which, status, bound, p, q, s, xn, pr, ompr) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn chdtriv(p: f64, x: f64) -> f64 {
    let mut p = p;
    let mut x = x;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut df: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(chdtriv_, 2, true, which, status, bound, p, q, x, df) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn chndtr(x: f64, df: f64, nc: f64) -> f64 {
    let mut x = x;
    let mut df = df;
    let mut nc = nc;
    let mut which = 1;
    let mut q: f64 = 0.0;
    let mut p: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(chndtr_, 0, true, which, status, bound, p, q, x, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn chndtrix(p: f64, df: f64, nc: f64) -> f64 {
    let mut p = p;
    let mut df = df;
    let mut nc = nc;
    let mut which = 2;
    let mut q: f64 = 1.0 - p;
    let mut x: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(chndtr_, 2, false, which, status, bound, p, q, x, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn chndtrdf(x: f64, p: f64, nc: f64) -> f64 {
    let mut x = x;
    let mut p = p;
    let mut nc = nc;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut df: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(chndtr_, 3, true, which, status, bound, p, q, x, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn chndtrnc(x: f64, df: f64, p: f64) -> f64 {
    let mut x = x;
    let mut df = df;
    let mut p = p;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut nc: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(chndtr_, 4, true, which, status, bound, p, q, x, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn fdtrf(p: f64, dfd: f64, f: f64) -> f64 {
    let mut p = p;
    let mut dfd = dfd;
    let mut f = f;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut dfn: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(fdtr_, 2, true, which, status, bound, p, q, f, dfn, dfd) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn fdtri(dfn: f64, p: f64, f: f64) -> f64 {
    let mut dfn = dfn;
    let mut p = p;
    let mut f = f;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut dfd: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(fdtr_, 4, true, which, status, bound, p, q, f, dfn, dfd) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn ncfdtrdfn(dfn: f64, dfd: f64, nc: f64, f: f64) -> f64 {
    let mut dfn = dfn;
    let mut dfd = dfd;
    let mut nc = nc;
    let mut f = f;
    let mut which = 1;
    let mut q: f64 = 0.0;
    let mut p: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(ncfdtr_, 0, false, which, status, bound, p, q, f, dfn, dfd, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn ncfdtrdfd(dfn: f64, dfd: f64, nc: f64, p: f64) -> f64 {
    let mut dfn = dfn;
    let mut dfd = dfd;
    let mut nc = nc;
    let mut p = p;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut f: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(ncfdtr_, 2, true, which, status, bound, p, q, f, dfn, dfd, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn ncfdtr(p: f64, dfd: f64, nc: f64, f: f64) -> f64 {
    let mut p = p;
    let mut dfd = dfd;
    let mut nc = nc;
    let mut f = f;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut dfn: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(ncfdtr_, 3, true, which, status, bound, p, q, f, dfn, dfd, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn ncfdtri(dfn: f64, p: f64, nc: f64, f: f64) -> f64 {
    let mut dfn = dfn;
    let mut p = p;
    let mut nc = nc;
    let mut f = f;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut dfd: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(ncfdtr_, 4, true, which, status, bound, p, q, f, dfn, dfd, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn ncfdtridfn(dfn: f64, dfd: f64, p: f64, f: f64) -> f64 {
    let mut dfn = dfn;
    let mut dfd = dfd;
    let mut p = p;
    let mut f = f;
    let mut which = 5;
    let mut q = 1.0 - p;
    let mut nc: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(ncfdtr_, 5, true, which, status, bound, p, q, f, dfn, dfd, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn gdtr(scl: f64, shp: f64, x: f64) -> f64 {
    let mut scl = scl;
    let mut shp = shp;
    let mut x = x;
    let mut which = 1;
    let mut q: f64 = 0.0;
    let mut p: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(gdtr_, 0, false, which, status, bound, p, q, x, shp, scl) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn gdtrix(scl: f64, shp: f64, p: f64) -> f64 {
    let mut scl = scl;
    let mut shp = shp;
    let mut p = p;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut x: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(gdtr_, 2, true, which, status, bound, p, q, x, shp, scl) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn gdtrib(scl: f64, p: f64, x: f64) -> f64 {
    let mut scl = scl;
    let mut p = p;
    let mut x = x;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut shp: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(gdtr_, 3, true, which, status, bound, p, q, x, shp, scl) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn gdtria(p: f64, shp: f64, x: f64) -> f64 {
    let mut p = p;
    let mut shp = shp;
    let mut x = x;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut scl: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(gdtr_, 4, true, which, status, bound, p, q, x, shp, scl) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn nbdtrik(p: f64, xn: f64, pr: f64) -> f64 {
    let mut p = p;
    let mut xn = xn;
    let mut pr = pr;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut s: f64 = 0.0;
    let mut ompr = 1.0 - pr;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(nbdtr_, 2, true, which, status, bound, p, q, s, xn, pr, ompr) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn nbdtrin(s: f64, p: f64, pr: f64) -> f64 {
    let mut s = s;
    let mut p = p;
    let mut pr = pr;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut xn: f64 = 0.0;
    let mut ompr = 1.0 - pr;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(nbdtr_, 3, true, which, status, bound, p, q, s, xn, pr, ompr) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn nrdtrimn(p: f64, std: f64, x: f64) -> f64 {
    let mut p = p;
    let mut std = std;
    let mut x = x;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut mn: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(nrdtr_, 3, true, which, status, bound, p, q, x, mn, std) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn nrdtrisd(mn: f64, p: f64, x: f64) -> f64 {
    let mut mn = mn;
    let mut p = p;
    let mut x = x;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut std: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(nrdtr_, 4, true, which, status, bound, p, q, x, mn, std) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn pdtrik(p: f64, xlam: f64) -> f64 {
    let mut p = p;
    let mut xlam = xlam;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut s: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(pdtr_, 2, true, which, status, bound, p, q, s, xlam) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn stdtr(df: f64, t: f64) -> f64 {
    let mut df = df;
    let mut t = t;
    let mut which = 1;
    let mut q: f64 = 0.0;
    let mut p: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(stdtr_, 0, false, which, status, bound, p, q, t, df) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn stdtrit(df: f64, p: f64) -> f64 {
    let mut df = df;
    let mut p = p;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut t: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(stdtr_, 2, true, which, status, bound, p, q, t, df) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn stdtridf(p: f64, t: f64) -> f64 {
    let mut p = p;
    let mut t = t;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut df: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(stdtr_, 3, true, which, status, bound, p, q, t, df) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn nctdtr(df: f64, nc: f64, t: f64) -> f64 {
    let mut df = df;
    let mut nc = nc;
    let mut t = t;
    let mut which = 1;
    let mut q: f64 = 0.0;
    let mut p: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(nctdtr_, 0, true, which, status, bound, p, q, t, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn nctdtrit(df: f64, nc: f64, p: f64) -> f64 {
    let mut df = df;
    let mut nc = nc;
    let mut p = p;
    let mut which = 2;
    let mut q = 1.0 - p;
    let mut t: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(nctdtr_, 2, true, which, status, bound, p, q, t, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn nctdtridf(p: f64, nc: f64, t: f64) -> f64 {
    let mut p = p;
    let mut nc = nc;
    let mut t = t;
    let mut which = 3;
    let mut q = 1.0 - p;
    let mut df: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(nctdtr_, 3, true, which, status, bound, p, q, t, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

pub fn nctdtrinc(df: f64, p: f64, t: f64) -> f64 {
    let mut df = df;
    let mut p = p;
    let mut t = t;
    let mut which = 4;
    let mut q = 1.0 - p;
    let mut nc: f64 = 0.0;
    let mut bound: f64 = 0.0;
    let mut status = 10;
    match call_cdflib_safe!(nctdtr_, 4, true, which, status, bound, p, q, t, df, nc) {
        Ok(res) => res,
        Err(msg) => { eprintln!("{}", msg); std::f64::NAN }
    }
}

fn main() {
}
