use ndarray::{Array, Axis, Dimension};
use num::complex::Complex;
use std::f64::consts::PI;

pub type C64 = Complex<f64>;

const SPEED: C64 = C64 {
    re: 0f64,
    im: -2.0 * PI,
};

pub fn to_c64(vs: Vec<f64>) -> Vec<C64> {
    vs.iter().map(|v| C64::new(*v, 0f64)).collect()
}

/// 1D Fast Fourier Transform
pub fn fft(vs: &Vec<C64>) -> Vec<C64> {
    let n = vs.len();
    if n == 1 {
        return vs.clone();
    } else {
        let evens = (*vs)
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, &v)| v)
            .collect();
        let odds = (*vs)
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, &v)| v)
            .collect();
        let f_even = fft(&evens);
        let f_odd = fft(&odds);
        let mut combined: Vec<C64> = vec![C64::new(0f64, 0f64); n];
        let n_f64 = n as f64;
        for k in 0..n / 2 {
            let k_f64 = k as f64;
            combined[k] = f_even[k] + f_odd[k] * (SPEED * k_f64 / n_f64);
            combined[k + n / 2] = f_even[k] - f_odd[k] * (SPEED * k_f64 / n_f64)
        }
        combined
    }
}

//fn mdfft<D: Dimension>(vs: &Array<C64, D>) -> Array<C64, D> {
//    let mut fs = vs.clone();
//    for d in 0..vs.dim() {
//        let size = fs.len_of(Axis(d));
//        let period = size as f64;
//        for mut lane in fs.lanes_mut(Axis(d)) {
//        }
//    }
//}

/// Naive 1D Discrete Fourier Transform
fn dft(vs: &Vec<C64>) -> Vec<C64> {
    let mut fs: Vec<C64> = Vec::new();
    let period = vs.len() as f64;
    for k in 0..vs.len() {
        let k = k as f64;
        let mut f = C64::new(0f64, 0f64);
        for (n, v) in vs.iter().enumerate() {
            let n = n as f64;
            f += v * (SPEED * k * n / period).exp();
        }
        fs.push(f);
    }
    fs
}

/// Naive ND Discrete Fourier Transform
fn mddft<D: Dimension>(vs: &Array<C64, D>) -> Array<C64, D> {
    let mut fs = vs.clone();
    for d in 0..vs.ndim() {
        let size = fs.len_of(Axis(d));
        let period = size as f64;
        for mut lane in fs.lanes_mut(Axis(d)) {
            let mut row: Vec<C64> = Vec::new();
            for k in 0..size {
                let k = k as f64;
                let mut f = C64::new(0f64, 0f64);
                for (n, v) in lane.iter().enumerate() {
                    let n = n as f64;
                    f += v * (SPEED * k * n / period).exp();
                }
                row.push(f);
            }
            let mut row_iter = row.iter();
            for v in lane.iter_mut() {
                *v = *row_iter.next().unwrap();
            }
        }
    }
    fs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //        let samples = to_c64(&samples)[..128].to_vec();
        //        let freqs_1d_dft = dft(&samples);
        //        let freqs_1d_fft = fft(&samples);
        //        let samples_arr = Array::from_vec(samples_vec);
        //        let freqs_md = mddft(&samples_arr);
    }
}
