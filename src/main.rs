mod amos;
mod cdflib;
mod cephes;
mod complex;
mod specfun;

use crate::amos::*;
use crate::cdflib::*;
use crate::cephes::*;
use crate::specfun::*;

fn main() {
    println!("{}", airy_a(1 as f64));
    println!("{}", der_airy_a(1 as f64));
    println!("{}", airy_b(1 as f64));
    println!("{}", der_airy_b(1 as f64));
    println!("{:?}", airy_a_zeros(3));
    println!("{:?}", airy_b_zeros(3));
    println!("{:?}", it_airy(3.0));
}
