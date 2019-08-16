use hound;
use num::complex::Complex;
use std::f64::consts::PI;

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
    let freqs = dft(&samples_c64[..100]);

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

/// Naive Discrete Fourier Transform
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
