use crate::complex::Complex;

extern "C" {
    fn zairy_(
        zr: *mut f64,
        zi: *mut f64,
        id: *mut i32,
        kode: *mut i32,
        air: *mut f64,
        aii: *mut f64,
        nz: *mut i32,
        ierr: *mut i32,
    );
}

macro_rules! fortran_call {
    ($func:ident($($arg:expr),*)) => { $func($(&mut $arg),*) }
}

pub fn airy_a<T: Complex>(z: T) -> T {
    let mut zr = z.real();
    let mut zi = z.imag();
    let mut id = 0;
    let mut kode = 1;
    let mut air = 0.0f64;
    let mut aii = 0.0f64;
    let mut nz = 0;
    let mut ierr = 0;
    unsafe { fortran_call!(zairy_(zr, zi, id, kode, air, aii, nz, ierr)); }
    println!("{}", ierr);
    T::from_tuple((air, aii))
}
