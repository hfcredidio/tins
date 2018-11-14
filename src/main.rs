mod amos;
mod cdflib;
mod cephes;
mod complex;

use amos::{airy, biry, dairy, dbiry};
use cdflib::*;
use cephes::i0;

fn main() {
    println!("{}", airy(1 as f64));
    println!("{}", dairy(1 as f64));
    println!("{}", biry(1 as f64));
    println!("{}", dbiry(1 as f64));
}
