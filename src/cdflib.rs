extern "C" {
    fn cdfbet(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        x: &mut f64,
        y: &mut f64,
        a: &mut f64,
        b: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );
    fn cdfbin(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        s: &mut f64,
        xn: &mut f64,
        pr: &mut f64,
        ompr: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );
    fn cdfchi(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        x: &mut f64,
        df: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );
    fn cdfchn(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        x: &mut f64,
        df: &mut f64,
        pnonc: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );
    fn cdff(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        f: &mut f64,
        dfn: &mut f64,
        dfd: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );
    fn cdffnc(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        f: &mut f64,
        dfn: &mut f64,
        dfd: &mut f64,
        phonc: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );
    fn cdfgam(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        x: &mut f64,
        shape: &mut f64,
        scale: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );
    fn cdfnbn(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        s: &mut f64,
        xn: &mut f64,
        pr: &mut f64,
        ompr: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );
    fn cdfnor(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        x: &mut f64,
        mean: &mut f64,
        sd: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );

    fn cdfpoi(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        s: &mut f64,
        xlam: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );
    fn cdft(
        which: &mut i32,
        p: &mut f64,
        q: &mut f64,
        t: &mut f64,
        df: &mut f64,
        status: &mut i32,
        bound: &mut f64,
    );
    fn cumbet(x: &mut f64, y: &mut f64, a: &mut f64, b: &mut f64, cum: &mut f64, ccum: &mut f64);
    fn cumbin(
        s: &mut f64,
        xn: &mut f64,
        pr: &mut f64,
        ompr: &mut f64,
        cum: &mut f64,
        ccum: &mut f64,
    );
    fn cumchi(x: &mut f64, df: &mut f64, cum: &mut f64, ccum: &mut f64);
    fn cumchn(x: &mut f64, df: &mut f64, pnonc: &mut f64, cum: &mut f64, ccum: &mut f64);
    fn cumf(f: &mut f64, dfn: &mut f64, dfd: &mut f64, cum: &mut f64, ccum: &mut f64);
    fn cumfnc(
        f: &mut f64,
        dfn: &mut f64,
        dfd: &mut f64,
        pnonc: &mut f64,
        cum: &mut f64,
        ccum: &mut f64,
    );
    fn cumgam(x: &mut f64, a: &mut f64, cum: &mut f64, ccum: &mut f64);
    fn cumnbn(
        s: &mut f64,
        xn: &mut f64,
        pr: &mut f64,
        ompr: &mut f64,
        cum: &mut f64,
        ccum: &mut f64,
    );
    fn cumnor(arg: &mut f64, result: &mut f64, ccum: &mut f64);
    fn cumpoi(s: &mut f64, xlam: &mut f64, cum: &mut f64, ccum: &mut f64);
    fn cumt(t: &mut f64, df: &mut f64, cum: &mut f64, ccum: &mut f64);
}


macro_rules! call_cdflib_cdf_safe {
    ($func:ident, $( $arg:ident ),*) => {
        if $( $arg.is_nan() )|* {
            Err("asdasd".to_string())
        } else {
            $(let mut $arg = $arg;)*
            let mut cdf = 0.0;
            let mut sf = 0.0;
            unsafe { $func($( &mut $arg ),*, &mut cdf, &mut sf); }
            Ok((cdf, sf))
        }
    };
}

extern crate std;

pub fn beta_cdf(x: f64, a: f64, b: f64) -> (f64, f64) {
    let y = 1.0 - x;
    match call_cdflib_cdf_safe!(cumbet, x, y, a, b) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}

pub fn binomial_cdf(s: f64, xn: f64, pr: f64) -> (f64, f64) {
    let ompr = 1.0 - pr;
    match call_cdflib_cdf_safe!(cumbin, s, xn, pr, ompr) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}

pub fn chi2_cdf(x: f64, df: f64) -> (f64, f64) {
    match call_cdflib_cdf_safe!(cumchi, x, df) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}

pub fn chi2nc_cdf(x: f64, df: f64, pnonc: f64) -> (f64, f64) {
    match call_cdflib_cdf_safe!(cumchn, x, df, pnonc) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}

pub fn f_cdf(f: f64, dfn: f64, dfd: f64) -> (f64, f64) {
    match call_cdflib_cdf_safe!(cumf, f, dfn, dfd) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}

pub fn fnc_cdf(f: f64, dfn: f64, dfd: f64, pnonc: f64) -> (f64, f64) {
    match call_cdflib_cdf_safe!(cumfnc, f, dfn, dfd, pnonc) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}

pub fn gamma_cdf(x: f64, a: f64) -> (f64, f64) {
    match call_cdflib_cdf_safe!(cumgam, x, a) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}

pub fn negbinomial_cdf(s: f64, xn: f64, pr: f64) -> (f64, f64) {
    let ompr = 1.0 - pr;
    match call_cdflib_cdf_safe!(cumnbn, s, xn, pr, ompr) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}

pub fn normal_cdf(x: f64) -> (f64, f64) {
    match call_cdflib_cdf_safe!(cumnor, x) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}

pub fn poisson_cdf(s: f64, xlam: f64) -> (f64, f64) {
    match call_cdflib_cdf_safe!(cumpoi, s, xlam) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}

pub fn t_cdf(t: f64, df: f64) -> (f64, f64) {
    match call_cdflib_cdf_safe!(cumt, t, df) {
        Err(msg) => { eprintln!("{}", msg); (std::f64::NAN, std::f64::NAN) },
        Ok(res) => res,
    }
}
