use num::complex::Complex64;
use plotters::prelude::*;
use rand;
use rand::Rng;
use itertools_num;
use itertools_num::ItertoolsNum;

pub fn plot_flow(flow: Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("target/plots/flow.png", (1024, 512)).into_drawing_area();
    root.fill(&WHITE);
    let n = flow.len() as i32;
    let mut chart = ChartBuilder::on(&root)
        .build_ranged(0..n, 0..n)?;
    chart.draw_series(
        flow.iter()
            .cumsum::<usize>()
            .zip(flow.iter())
            .enumerate()
            .map(|(x, (p, &l))| (x as i32, p as i32, l as i32))
            .map(|(x, p, l)| {
                Rectangle::new(
                    [(p - l, x), (p, x + 1)],
                    HSLColor(0.0, 0.0, 0.5).filled()
                )
            })
    )?;
    Ok(())
}

pub fn plot_matrix(matrix: Vec<Vec<f64>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("target/plots/similarity.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;
    let n = matrix.len() as i32;
    let mut chart = ChartBuilder::on(&root)
        .build_ranged(0..n, n..0)?;
    chart.draw_series(
        matrix.iter().enumerate()
            .map(|(i, v)| v.iter().enumerate()
                .map(move |(j, c)| (i as i32, j as i32, c.ln_1p())))
            .flatten()
            .map(|(x, y, v)|
                Rectangle::new(
                    [(x, y), (x + 1, y + 1)],
                    HSLColor(0.0, 0.0, v).filled(),
                )
            ),
    )?;
    Ok(())
}

pub fn plot_spectrum(stft: Vec<Vec<Complex64>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("target/plots/spectrum.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (length, height) = (stft.len(), stft.first().unwrap().len() / 2);
    let mut chart = ChartBuilder::on(&root)
        .build_ranged(0..length as i32, height as i32..0)?;
    chart.draw_series(
        stft.iter().enumerate()
            .map(|(i, v)| v.split_at(height).1.iter().enumerate()
                .map(move |(j, c)| (i as i32, j as i32, c.norm().ln_1p())))
            .flatten()
            .map(|(x, y, v)| {
                Rectangle::new(
                    [(x, y), (x + 1, y + 1)],
                    HSLColor(0.0, 0.0, v).filled(),
                )
            }),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader;
    use crate::fourier;
    use std::error::Error;

    #[test]
    fn test_plot_spectrum() -> Result<(), Box<dyn Error>>{
        let stft = { // Block same as run in lib.rs
            let time_signal = loader::load_wav_samples(&"export.wav".to_string())?;
            let complex_signal = fourier::to_complex64(time_signal);
            let frequency_signal = fourier::fft(&complex_signal);
            complex_signal.chunks(256)
                .map(|chunk| fourier::fft(&chunk.to_vec()))
                .collect()
        };
        plot_spectrum(stft)?;
        Ok(())
    }

    #[test]
    fn test_plot_similarity() -> Result<(), Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let mut matrix: Vec<Vec<f64>> = Vec::new();
        for i in 0..100 {
            matrix.push(Vec::new());
            for j in 0..100 {
                let v: f64 = rng.gen();
                matrix.last_mut().unwrap().push(v);
            }
        }
//        let matrix: Vec<Vec<f64>> = vec![vec![rng.gen(); 100]; 100];
        plot_matrix(matrix)
    }

    #[test]
    fn test_plot_flow() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let flow = (0..100).map(|_| rng.gen_range(1, 5)).collect();
//        let flow = vec![rng.gen_range(1, 5); 100];
        plot_flow(flow)
    }
}
