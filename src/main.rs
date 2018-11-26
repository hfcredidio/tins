mod amos;
mod cdflib;
mod cephes;
mod complex;
mod specfun;

use std::io::Read;
use std::mem::transmute;

macro_rules! read {
    ($n:expr, $file:ident) => {{
        let mut buf = [0u8; $n];
        $file.read(&mut buf).unwrap();
        unsafe { transmute(buf) }
    }};
}

fn read_file(file: &mut std::fs::File) {
    let x: [f64; 100] = read!(8 * 100, file);
    let y: [f64; 100] = read!(8 * 100, file);
    for i in 0.. 100 {
        println!("{:?}, {:?}", x[i], y[i]);
    }
}

fn main() {
    let mut file = std::fs::File::open("/home/heitor/rust/tins/src/test_data/airy_a.dat").unwrap();
    read_file(&mut file);
}
