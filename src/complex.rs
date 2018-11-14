pub struct c64(f64, f64);

pub trait Complex {
    fn from_tuple((f64, f64)) -> Self;
    fn real(&self) -> f64;
    fn imag(&self) -> f64;
}

impl Complex for c64 {
    fn from_tuple(z: (f64, f64)) -> c64 {
        c64(z.0, z.1)
    }
    fn real(&self) -> f64 {
        self.0
    }
    fn imag(&self) -> f64 {
        self.1
    }
}

impl Complex for f64 {
    fn from_tuple(z: (f64, f64)) -> f64 {
        z.0
    }
    fn real(&self) -> f64 {
        *self
    }
    fn imag(&self) -> f64 {
        0.0
    }
}
