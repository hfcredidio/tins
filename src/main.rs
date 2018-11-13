mod cdflib;
mod cephes;

use cdflib::*;
use cephes::{i0};


fn main() {
    for i in 1..10 {
        println!("{}", i0(i as f64));
    }
}
