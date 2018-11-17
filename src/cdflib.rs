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
        pnonc: &mut f64,
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
}

fn get_result(
    name: &str,
    status: i32,
    result: f64,
    bound: f64,
    return_bound: bool,
) -> Result<f64, String> {
    match status {
        _ if status < 0 => Err(format!(
            "cdflib.{}: Argument {} is out of bounds",
            name, -result
        )),
        0 => Ok(result),
        1 | 2 if return_bound => Ok(bound),
        1 => Err(format!(
            "cdflib.{}: the answer appears to be lower than lowest search bound",
            name
        )),
        2 => Err(format!(
            "cdflib.{}: the answer appears to be higher than greatest search bound",
            name
        )),
        10 => Err(format!("cdflib.{}: Computation error.", name)),
        _ => Err(format!("cdflib.{}: Unknown error.", name)),
    }
}

macro_rules! _cdflib {
    (@get_result_var $which:expr, $($arg:ident),*) => {
        [$($arg),*][($which - 1) as usize]
    };

    (@call cdfbet, $which:expr, $p:ident, $x:ident, $($arg:ident),*) => {{
        let mut which = $which;
        let mut p: f64 = $p;
        let mut q = 1.0 - p;
        let mut x = $x;
        let mut y = 1.0 - x;
        $(let mut $arg = $arg;)*
        let mut status = 0;
        let mut bound = 0.0f64;
        unsafe { cdfbet(&mut which, &mut p, &mut q, &mut x, &mut y, $(&mut $arg),*, &mut status, &mut bound); }
        let result = _cdflib!(@get_result_var $which, $p, $x, $($arg),*);
        get_result("cdfbet", status, result, bound, true)
    }};

    (@call cdfbin, $which:expr, $p:ident, $pr:ident, $($arg:ident),*) => {{
        let mut which = $which;
        let mut p: f64 = $p;
        let mut q = 1.0 - p;
        $(let mut $arg = $arg;)*
        let mut pr = $pr;
        let mut ompr = 1.0 - pr;
        let mut status = 0;
        let mut bound = 0.0f64;
        unsafe { cdfbet(&mut which, &mut p, &mut q, $(&mut $arg),*, &mut pr, &mut ompr, &mut status, &mut bound); }
        let result = _cdflib!(@get_result_var $which, p, $($arg),*, pr);
        get_result("cdfbin", status, result, bound, true)
    }};

    (@call cdfnbn, $which:expr, $p:ident, $pr:ident, $($arg:ident),*) => {{
        let mut which = $which;
        let mut p: f64 = $p;
        let mut q = 1.0 - p;
        $(let mut $arg = $arg;)*
        let mut pr = $pr;
        let mut ompr = 1.0 - pr;
        let mut status = 0;
        let mut bound = 0.0f64;
        unsafe { cdfnbn(&mut which, &mut p, &mut q, $(&mut $arg),*, &mut pr, &mut ompr, &mut status, &mut bound); }
        let result = _cdflib!(@get_result_var $which, p, $($arg),*, pr);
        get_result("cdfnbn", status, result, bound, true)
    }};

    (@call $func:ident, $which:expr, $p:ident, $($arg:ident),*) => {{
        let mut which = $which;
        let mut p: f64 = $p;
        let mut q = 1.0 - p;
        $(let mut $arg = $arg;)*
        let mut status = 0;
        let mut bound = 0.0f64;
        unsafe { $func(&mut which, &mut p, &mut q, $(&mut $arg),*, &mut status, &mut bound); }
        let result = _cdflib!(@get_result_var $which, p, $($arg),*);
        get_result(stringify!($func), status, result, bound, true)
    }};

    (@call_safe $func:ident, $which:expr, $($arg:ident),*) => {
        if $($arg.is_nan())|* {
            Err(format!("cdflib.{}: NAN passed as argument.", stringify!($func)))
        } else {
            _cdflib!(@call $func, $which, $($arg),*)
        }
    };

    (@call_nan $func:ident, $which:expr, $($arg:ident),*) => {
        match _cdflib!(@call_safe $func, $which, $($arg),*) {
            Err(msg) => { eprintln!("{}", msg); std::f64::NAN },
            Ok(res) => res,
        }
    };

    (@which cdfchn, pnonc) => { 4 };
    (@which cdffnc, pnonc) => { 5 };

    (@which $func:ident, p) => { 1 };

    (@which $func:ident, x) => { 2 };
    (@which $func:ident, s) => { 2 };
    (@which $func:ident, f) => { 2 };
    (@which $func:ident, t) => { 2 };

    (@which $func:ident, a) => { 3 };
    (@which $func:ident, xn) => { 3 };
    (@which $func:ident, df) => { 3 };
    (@which $func:ident, dfn) => { 3 };
    (@which $func:ident, shape) => { 3 };
    (@which $func:ident, mean) => { 3 };
    (@which $func:ident, xlam) => { 3 };

    (@which $func:ident, b) => { 4 };
    (@which $func:ident, pr) => { 4 };
    (@which $func:ident, dfd) => { 4 };
    (@which $func:ident, scale) => { 4 };
    (@which $func:ident, sd) => { 4 };

    (@wrap $name:ident($($arg:ident),*) -> $ret:ident wraps $func:ident($($ordered_args:ident),*)) => {
        pub fn $name($($arg: f64,)*) -> f64 {
            let $ret: f64 = 0.0;
            _cdflib!(@call_nan $func, _cdflib!(@which $func, $ret), $($ordered_args),*)
        }
    };
}

_cdflib!(@wrap beta_cdf(x, a, b) -> p wraps cdfbet(p, x, a, b));
_cdflib!(@wrap beta_icdf_x(p, a, b) -> x wraps cdfbet(p, x, a, b));
_cdflib!(@wrap beta_icdf_a(p, x, b) -> a wraps cdfbet(p, x, a, b));
_cdflib!(@wrap beta_icdf_b(p, x, a) -> b wraps cdfbet(p, x, a, b));

_cdflib!(@wrap binomial_cdf(s, xn, pr) -> p wraps cdfbet(p, pr, s, xn));
_cdflib!(@wrap binomial_icdf_s(p, xn, pr) -> s wraps cdfbet(p, pr, s, xn));
_cdflib!(@wrap binomial_icdf_xn(p, s, pr) -> xn wraps cdfbet(p, pr, s, xn));
_cdflib!(@wrap binomial_icdf_pr(p, s, xn) -> pr wraps cdfbet(p, pr, s, xn));

_cdflib!(@wrap chi2_cdf(x, df) -> p wraps cdfchi(p, x, df));
_cdflib!(@wrap chi2_icdf_x(p, df) -> x wraps cdfchi(p, x, df));
_cdflib!(@wrap chi2_icdf_df(p, x) -> df wraps cdfchi(p, x, df));

_cdflib!(@wrap chi2nc_cdf(x, df, pnonc) -> p wraps cdfchn(p, x, df, pnonc));
_cdflib!(@wrap chi2nc_icdf_x(p, df, pnonc) -> x wraps cdfchn(p, x, df, pnonc));
_cdflib!(@wrap chi2nc_icdf_df(p, x, pnonc) -> df wraps cdfchn(p, x, df, pnonc));
_cdflib!(@wrap chi2nc_icdf_pnonc(p, x, df) -> pnonc wraps cdfchn(p, x, df, pnonc));

_cdflib!(@wrap f_cdf(f, dfn, dfd) -> p wraps cdff(p, f, dfn, dfd));
_cdflib!(@wrap f_icdf_f(p, dfn, dfd) -> f wraps cdff(p, f, dfn, dfd));
_cdflib!(@wrap f_icdf_dfn(p, f, dfd) -> dfn wraps cdff(p, f, dfn, dfd));
_cdflib!(@wrap f_icdf_dfd(p, f, dfn) -> dfd wraps cdff(p, f, dfn, dfd));

_cdflib!(@wrap fnc_cdf(f, dfn, dfd, pnonc) -> p wraps cdffnc(p, f, dfn, dfd, pnonc));
_cdflib!(@wrap fnc_icdf_f(p, dfn, dfd, pnonc) -> f wraps cdffnc(p, f, dfn, dfd, pnonc));
_cdflib!(@wrap fnc_icdf_dfn(p, f, dfd, pnonc) -> dfn wraps cdffnc(p, f, dfn, dfd, pnonc));
_cdflib!(@wrap fnc_icdf_dfd(p, f, dfn, pnonc) -> dfd wraps cdffnc(p, f, dfn, dfd, pnonc));
_cdflib!(@wrap fnc_icdf_pnonc(p, f, dfn, dfd) -> pnonc wraps cdffnc(p, f, dfn, dfd, pnonc));

_cdflib!(@wrap gamma_cdf(x, shape, scale) -> p wraps cdfgam(p, x, shape, scale));
_cdflib!(@wrap gamma_icdf_x(p, shape, scale) -> x wraps cdfgam(p, x, shape, scale));
_cdflib!(@wrap gamma_icdf_shape(p, x, scale) -> shape wraps cdfgam(p, x, shape, scale));
_cdflib!(@wrap gamma_icdf_scale(p, x, shape) -> scale wraps cdfgam(p, x, shape, scale));

_cdflib!(@wrap negbinomial_cdf(s, xn, pr) -> p wraps cdfnbn(p, pr, s, xn));
_cdflib!(@wrap negbinomial_icdf_s(p, xn, pr) -> s wraps cdfnbn(p, pr, s, xn));
_cdflib!(@wrap negbinomial_icdf_xn(p, s, pr) -> xn wraps cdfnbn(p, pr, s, xn));
_cdflib!(@wrap negbinomial_icdf_pr(p, s, xn) -> pr wraps cdfnbn(p, pr, s, xn));

_cdflib!(@wrap normal_cdf(x, mean, sd) -> p wraps cdfnor(p, x, mean, sd));
_cdflib!(@wrap normal_icdf_x(p, mean, sd) -> x wraps cdfnor(p, x, mean, sd));
_cdflib!(@wrap normal_icdf_mean(p, x, sd) -> mean wraps cdfnor(p, x, mean, sd));
_cdflib!(@wrap normal_icdf_sd(p, x, mean) -> sd wraps cdfnor(p, x, mean, sd));

_cdflib!(@wrap poisson_cdf(s, xlam) -> p wraps cdfpoi(p, s, xlam));
_cdflib!(@wrap poisson_icdf_s(p, xlam) -> s wraps cdfpoi(p, s, xlam));
_cdflib!(@wrap poisson_icdf_xlam(p, s) -> xlam wraps cdfpoi(p, s, xlam));

_cdflib!(@wrap t_cdf(t, df) -> p wraps cdft(p, t, df));
_cdflib!(@wrap t_icdf_t(p, df) -> t wraps cdft(p, t, df));
_cdflib!(@wrap t_icdf_df(p, t) -> df wraps cdft(p, t, df));
