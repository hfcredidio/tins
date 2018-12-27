extern crate byteorder;

mod airy;
mod amos;
mod cdflib;
mod cephes;
mod complex;
mod specfun;

use byteorder::{ByteOrder, LittleEndian};
use complex::Complex;
use std::fmt::Debug;
use std::io::Read;


fn read_floats(path: &str) -> Vec<f64> {
    let mut buf = vec![];
    std::fs::File::open(path).unwrap().read_to_end(&mut buf).unwrap();
    let mut data = vec![0.0f64; buf.len() / 8];
    LittleEndian::read_f64_into_unchecked(&buf, &mut data);
    data
}


fn read_complex(path: &str) -> Vec<complex::c64> {
    read_floats(path).chunks(2).map(|ri| {
        let real = ri[0];
        let imag = ri[1];
        complex::c64(real, imag)
    }).collect()
}


fn almost_eq(x: f64, y: f64, tol: f64) -> bool  {
    if x.is_nan() && y.is_nan() {
        true
    } else {
        let err = if x.abs() < 1e-10 { (x - y).abs() } else { (x - y).abs() / x.abs() };
        err < tol
    }
}


fn assert_almost_eq<T: complex::Complex + Debug>(x: T, y: T, z: T, tol: f64) {
    assert!(almost_eq(y.real(), z.real(), tol) && almost_eq(y.imag(), z.imag(), tol), "{:?} {:?} {:?}", x, y, z);
}


fn main() {
    //for xy in read_floats("./src/test_data/airy_a_f64.dat").chunks(2) {
        //let x = xy[0];
        //let y = xy[1];
        //let z = airy::airy_a(x);
        //assert_almost_eq(x, y, z, 1e-5);
    //}
    for xy in read_complex("./src/test_data/airy_a_c64.dat").chunks(2) {
        let x = xy[0];
        let y = xy[1];
        let z = airy::airy_a(x);
        assert_almost_eq(x, y, z, 1e-5);
    }
}
