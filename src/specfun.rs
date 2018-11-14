extern "C" {
    fn airyzo_(
        nt: &mut i32,
        kr: &mut i32,
        xa: *mut f64,
        xb: *mut f64,
        xc: *mut f64,
        xd: *mut f64,
    );

    fn itairy_(
        x: &mut f64,
        apt: &mut f64,
        bpt: &mut f64,
        ant: &mut f64,
        bnt: &mut f64,
    );
}

fn airy_zeros(nt: i32, kf: i32) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut nt = nt;
    let mut kr = kf;
    let mut xa = vec![0.0f64; nt as usize];
    let mut xb = vec![0.0f64; nt as usize];
    let mut xc = vec![0.0f64; nt as usize];
    let mut xd = vec![0.0f64; nt as usize];
    unsafe { airyzo_(&mut nt, &mut kr, xa.as_mut_ptr(), xb.as_mut_ptr(), xc.as_mut_ptr(), xd.as_mut_ptr()); }
    (xa, xb, xc, xd)
}

pub fn airy_a_zeros(nt: i32) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) { airy_zeros(nt, 1) }
pub fn airy_b_zeros(nt: i32) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) { airy_zeros(nt, 2) }

pub fn it_airy(x: f64) -> (f64, f64, f64, f64) {
    let mut x = x;
    let mut apt = 0.0f64;
    let mut bpt = 0.0f64;
    let mut ant = 0.0f64;
    let mut bnt = 0.0f64;
    unsafe { itairy_(&mut x, &mut apt, &mut bpt, &mut ant, &mut bnt); }
    (apt, bpt, ant, bnt)
}
