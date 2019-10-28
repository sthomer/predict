use crate::abstraction::C64;
use crate::perception::{Dimension, Label, Location, Spectrum};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn categorize(dimension: &Dimension, spectrum: &Spectrum) -> Label {
    let mut candidates = Vec::new();
    for (label, Location { centroid, radius }) in &dimension.locations {
        if norm(centroid - spectrum.point) <= *radius {
            let ic = -(dimension.unigram[label] as f64 / dimension.total as f64).log2();
            candidates.push((label, ic))
        }
    }
    if candidates.is_empty() {
        generate_label()
    } else {
        candidates
            .into_iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0
            .clone()
    }
}

fn norm(point: C64) -> f64 {
    point.norm()
}

fn generate_label() -> Label {
    thread_rng().sample_iter(&Alphanumeric).take(10).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
