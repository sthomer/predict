use num::complex::Complex64;
use plotters::prelude::*;

pub fn plot_spectrum(stft: Vec<Vec<Complex64>>) -> Result<(), Box<dyn std::error::Error>>{
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
    fn test() -> Result<(), Box<dyn Error>>{

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
}
