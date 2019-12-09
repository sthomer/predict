use num::complex::Complex64;
use ndarray::{Array, Axis, Dimension};
use std::f64::consts::PI;
use ndarray::{Array1, ArrayView1, s, azip};
use ndarray_linalg::types::c64;

/// Convert from float vector to complex vector
pub fn to_complex64(a: Array1<f64>) -> Array1<Complex64> {
    a.map(|&v| c64::new(v, 0f64))
//    vs.iter().map(|v| Complex64::new(*v, 0f64)).collect()
}

/// 1D Fast Fourier Transform
pub fn fft(a: ArrayView1<c64>) -> Array1<c64> {
    let n = a.len();
    if n == 1 {
        a.into_owned()
    } else {
        let evens = fft(a.slice(s![..;2]));
        let odds = fft(a.slice(s![1..;2]));
        let mut comb = Array1::default(n);
        azip!((index k, &even in &evens, &odd in &odds) {
            let change = odd * (SPEED * k as f64 / n as f64);
            comb[k] = even + change;
            comb[k + n/2 as usize] = even - change;
        });
        comb
    }
}

//        let evens = (*vs).iter().enumerate()
//            .filter(|&(i, _)| i % 2 == 0)
//            .map(|(_, &v)| v)
//            .collect();
//        let odds = (*vs).iter().enumerate()
//            .filter(|&(i, _)| i % 2 != 0)
//            .map(|(_, &v)| v)
//            .collect();
//        let f_even = fft(&evens);
//        let f_odd = fft(&odds);
//        let mut combined: Vec<Complex64> = vec![Complex64::new(0f64, 0f64); n];
//        let n_f64 = n as f64;
//        for k in 0..n / 2 {
//            let k_f64 = k as f64;
//            combined[k] = f_even[k] + f_odd[k] * (SPEED * k_f64 / n_f64);
//            combined[k + n / 2] = f_even[k] - f_odd[k] * (SPEED * k_f64 / n_f64)
//        }
//        combined

const SPEED: Complex64 = Complex64 {
    re: 0f64,
    im: -2.0 * PI,
};

/// Naive 1D Discrete Fourier Transform
#[allow(dead_code)]
fn dft(vs: &Vec<Complex64>) -> Vec<Complex64> {
    let mut fs: Vec<Complex64> = Vec::new();
    let period = vs.len() as f64;
    for k in 0..vs.len() {
        let k = k as f64;
        let mut f = Complex64::new(0f64, 0f64);
        for (n, v) in vs.iter().enumerate() {
            let n = n as f64;
            f += v * (SPEED * k * n / period).exp();
        }
        fs.push(f);
    }
    fs
}

/// Naive MD Discrete Fourier Transform
#[allow(dead_code)]
fn mddft<D: Dimension>(vs: &Array<Complex64, D>) -> Array<Complex64, D> {
    let mut fs = vs.clone();
    for d in 0..vs.ndim() {
        let size = fs.len_of(Axis(d));
        let period = size as f64;
        for mut lane in fs.lanes_mut(Axis(d)) {
            let mut row: Vec<Complex64> = Vec::new();
            for k in 0..size {
                let k = k as f64;
                let mut f = Complex64::new(0f64, 0f64);
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