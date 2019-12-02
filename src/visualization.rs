use num::complex::Complex64;
use plotters::prelude::*;
use rand;
use rand::Rng;
use itertools_num;
use itertools_num::ItertoolsNum;

/// Type aliases for improved readability
type Res = Result<(), Box<dyn std::error::Error>>;
type X = usize;
type Y = usize;
type Index = usize;
type Length = usize;

/// Plot the points on a standard xy-coordinate plot.
///
/// # Arguments
/// * `points` - list of xy-coordinate points
///
pub fn plot_scatter(points: Vec<(X, Y)>) -> Res {
    let root = BitMapBackend::new("target/plots/scatter.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE);
    let max_x = points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|(_, y)| y).max().unwrap();
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0..*max_x, 0..*max_y)?;
    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .draw()?;
    chart.draw_series(
        PointSeries::of_element(points, 5, &BLACK, &|c, s, st|
            return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;
    Ok(())
}

/// Plot the lengths in a flow chart.
/// Plot the sequence of (index, length)'s in the given order on x-axis.
/// Index corresponds to the y-axis; Length corresponds to segment length.
///
/// # Arguments
/// * `flow` - sequence of (index, length) pairs
///
pub fn plot_flow(flow: Vec<(Index, Length)>) -> Res {
    let root = BitMapBackend::new("target/plots/flow.png", (1024, 512)).into_drawing_area();
    root.fill(&WHITE);
    let n = flow.len() as i32;
    let mut chart = ChartBuilder::on(&root)
        .build_ranged(0..n, 0..n)?;
    chart.draw_series(
        flow.iter()
            .map(|(_, length)| length)
            .cumsum::<usize>()
            .zip(flow.iter())
            .map(|(x, (y, l))| (x as i32, *y as i32, *l as i32))
            .map(|(x, y, l)| {
                Rectangle::new(
                    [(x - l, y), (x, y + 1)],
                    HSLColor(0.0, 0.0, 0.5).filled()
                )
            })
    )?;
    Ok(())
}

/// Plot the given matrix as a similarity matrix (i.e. heat map)
///
/// # Arguments
/// * `matrix` - square matrix of similarity values
///
pub fn plot_similarity(matrix: Vec<Vec<f64>>) -> Res {
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

/// Plot a short-term frequency spectrum as a spectrogram.
///
/// # Arguments
/// * `sftf` - short-term frequency spectrum matrix
///
pub fn plot_spectrum(stft: Vec<Vec<Complex64>>) -> Res {
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
    fn test_plot_spectrum() -> Res {
        let stft = { // Block same as run in lib.rs
            let time_signal = loader::load_wav(&"export.wav".to_string())?;
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
    fn test_plot_similarity() -> Res {
        let mut rng = rand::thread_rng();
        let matrix = (0..100)
            .map(|_| (0..100)
                .map(|_| rng.gen())
                .collect())
            .collect();
        plot_similarity(matrix)
    }

    #[test]
    fn test_plot_flow() -> Res {
        let mut rng = rand::thread_rng();
        let flow = (0..100)
            .map(|_| (rng.gen_range(1,100), rng.gen_range(1, 5)))
            .collect();
        plot_flow(flow)
    }

    #[test]
    fn test_plot_scatter() -> Res {
        let mut rng = rand::thread_rng();
        let scatter = (0..100)
            .map(|_| (rng.gen_range(0, 20), rng.gen_range(0, 20)))
            .collect();
        plot_scatter(scatter)
    }
}
