use hound;
use num::complex::Complex;
use std::f64::consts::PI;
use ndarray::{Array, Dimension, Axis};

type C64 = Complex<f64>;

const SPEED: C64 = C64 {re: 0f64, im: -2.0 * PI};

fn main() {
    let mut reader = hound::WavReader::open("/home/sthomer/rust/predict/export.wav").unwrap();
    let samples: Vec<i16> = reader.samples().filter_map(Result::ok).collect();
    let slides_1 = slides(&samples, 8, 4);
    let slides_1 = slides_1.collect::<Vec<_>>();
    let slides_2 = slides(&slides_1, 8, 4);
    let slides_2 = slides_2.collect::<Vec<_>>();
    let slides_3 = slides(&slides_2, 8, 4);
    let slides_3 = slides_3.collect::<Vec<_>>();

    let samples_c64 = to_c64(&samples);
    let freqs_1d_dft = dft(&samples_c64[..128]);
    let samples_vec = samples_c64[..128].to_vec();
    let freqs_1d_fft = fft(samples_vec);
//    let samples_arr = Array::from_vec(samples_vec);
//    let freqs_md = mddft(&samples_arr);

    println!(
        "1: {}\n2: {}\n3: {}",
        slides_1.len(),
        slides_2.len(),
        slides_3.len()
    );
}

fn to_c64(vs: &Vec<i16>) -> Vec<C64> {
    vs.iter().map(|v| {
        C64::new(*v as f64, 0f64)
    }).collect()
}

/// 1D Fast Fourier Transform
fn fft(vs: Vec<C64>) -> Vec<C64> {
    let n = vs.len();
    if n == 1 {
        return vs;
    } else {
        let evens = vs.iter().enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, &v)| v)
            .collect();
        let odds = vs.iter().enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, &v)| v)
            .collect();
        let f_even = fft(evens);
        let f_odd = fft(odds);
        let mut combined: Vec<C64> = vec![C64::new(0f64, 0f64); n];
        let n_f64 = n as f64;
        for k in 0..n/2 {
            let k_f64 = k as f64;
            combined[k] = f_even[k] + f_odd[k] * (SPEED * k_f64 / n_f64);
            combined[k + n/2] = f_even[k] - f_odd[k] * (SPEED * k_f64 / n_f64)
        }
        combined
    }
}

/// Naive 1D Discrete Fourier Transform
fn dft(vs: &[C64]) -> Vec<C64> {
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
        let period= size as f64;
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

struct Slides<'a, T: 'a> {
    v: &'a [T],
    chunk_size: usize,
    step_size: usize,
}

impl<'a, T> Iterator for Slides<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<&'a [T]> {
        if self.chunk_size > self.v.len() {
            None
        } else {
            let ret = Some(&self.v[..self.chunk_size]);
            self.v = &self.v[self.step_size..];
            ret
        }
    }
}

fn slides<T>(slice: &Vec<T>, chunk_size: usize, step_size: usize) -> Slides<T> {
    assert!(chunk_size != 0);
    assert!(step_size != 0);
    assert!(chunk_size >= step_size);
    Slides {
        v: slice,
        chunk_size,
        step_size,
    }
}