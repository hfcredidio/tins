mod _cephes {
    extern "C" {
        pub fn airy(x: f64, ai: &mut f64, aip: &mut f64, bi: &mut f64, bip: &mut f64) -> i32;
    }
}

pub fn airy(x: f64) -> (f64, f64, f64, f64) {
    let mut ai = 0.0f64;
    let mut bi = 0.0f64;
    let mut aip = 0.0f64;
    let mut bip = 0.0f64;
    let err = unsafe { _cephes::airy(x, &mut ai, &mut bi, &mut aip, &mut bip) };
    if err == -1 {
        eprintln!("cephes.airy: value of x too large");
    }
    (ai, bi, aip, bip)
}
