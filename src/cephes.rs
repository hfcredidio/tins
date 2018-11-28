extern "C" {
    #[link_name = "airy"]
    fn cephes_airy(x: f64, ai: *mut f64, aip: *mut f64, bi: *mut f64, bip: *mut f64);
}

pub fn airy(x: f64) -> (f64, f64, f64, f64) {
    let mut ai = 0.0f64;
    let mut aip = 0.0f64;
    let mut bi = 0.0f64;
    let mut bip = 0.0f64;
    unsafe {
        cephes_airy(x, &mut ai, &mut aip, &mut bi, &mut bip);
    }
    (ai, aip, bi, bip)
}
