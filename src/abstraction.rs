use crate::concept_symbol::Concept;
use ndarray::{Array, Axis, Dimension};
use num::complex::Complex64;
use std::f64::consts::PI;
use std::ops::Mul;

type Vector = Vec<Complex64>;

#[derive(Clone)]
pub struct Spectrum {
    pub point: Vector,
    pub length: usize,
}

pub fn transform(trajectory: Vec<&Concept>) -> Spectrum {
    let signal: Vec<Vector> = trajectory
        .iter()
        .map(|c| c.location.centroid.clone())
        .collect();
    let spectrum = fourier(signal);
    let vector = spectrum.into_iter().flatten().collect();
    Spectrum {
        point: vector,
        length: trajectory.len(),
    }
}

// TODO: Deal with all the clones
// TODO: Create struct for hold Vec<Vec<Complex>> and/or Vec<Complex>
pub fn fourier(signal: Vec<Vector>) -> Vec<Vector> {
    let n = signal.len();
    if n == 1 {
        return signal;
    } else {
        //        let (even, odd) = signal.into_iter().enumerate().partition(|&(i,_)| i % 2 == 0);
        let evens = signal
            .clone()
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, v)| v)
            .collect();
        let odds = signal
            .clone()
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, v)| v)
            .collect();
        let f_even = fourier(evens);
        let f_odd = fourier(odds);
        let mut combined: Vec<Vector> = vec![Vec::new(); n];
        for k in 0..n / 2 {
            let omega = SPEED * k as f64 / n as f64;
            let other: Vector = f_odd[k].clone().into_iter().map(|el| el * omega).collect();
            combined[k] = f_even[k]
                .clone()
                .iter()
                .zip(other.iter())
                .map(|(l, r)| l + r)
                .collect();
            combined[k + n / 2] = f_even[k]
                .clone()
                .iter()
                .zip(other.iter())
                .map(|(l, r)| l - r)
                .collect();
        }
        combined
    }
}

const SPEED: Complex64 = Complex64 {
    re: 0f64,
    im: -2.0 * PI,
};

pub fn to_complex64(vs: Vec<f64>) -> Vec<Complex64> {
    vs.iter().map(|v| Complex64::new(*v, 0f64)).collect()
}

/// 1D Fast Fourier Transform
pub fn fft(vs: &Vec<Complex64>) -> Vec<Complex64> {
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
        let mut combined: Vec<Complex64> = vec![Complex64::new(0f64, 0f64); n];
        let n_f64 = n as f64;
        for k in 0..n / 2 {
            let k_f64 = k as f64;
            combined[k] = f_even[k] + f_odd[k] * (SPEED * k_f64 / n_f64);
            combined[k + n / 2] = f_even[k] - f_odd[k] * (SPEED * k_f64 / n_f64)
        }
        combined
    }
}

//fn mdfft<D: Dimension>(vs: &Array<Complex64, D>) -> Array<Complex64, D> {
//    let mut fs = vs.clone();
//    for d in 0..vs.dim() {
//        let size = fs.len_of(Axis(d));
//        let period = size as f64;
//        for mut lane in fs.lanes_mut(Axis(d)) {
//        }
//    }
//}

/// Naive 1D Discrete Fourier Transform
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

/// Naive ND Discrete Fourier Transform
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //        let samples = to_Complex64(&samples)[..128].to_vec();
        //        let freqs_1d_dft = dft(&samples);
        //        let freqs_1d_fft = fft(&samples);
        //        let samples_arr = Array::from_vec(samples_vec);
        //        let freqs_md = mddft(&samples_arr);
    }
}
