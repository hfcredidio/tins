mod _cephes {
    extern "C" {
        pub fn i0(x: f64) -> f64;
    }
}

pub fn i0(x: f64) -> f64 {
    unsafe { _cephes::i0(x) }
}
