use crate::abstraction::C64;
use crate::Config;
use itertools::Itertools;
use crate::{abstraction, categorization, segmentation};

use std::collections::HashMap;

pub fn process(config: &Config, signal: Vec<C64>) -> Vec<Dimension> {
    let mut dimensions = vec![
        Dimension::new(0, 1 * config.radius_scale, config.resolution),
        Dimension::new(1, 10 * config.radius_scale, config.resolution),
        Dimension::new(2, 100 * config.radius_scale, config.resolution),
        Dimension::new(3, 1000 * config.radius_scale, config.resolution),
    ];

    for point in signal.into_iter() {
        perceive(&mut dimensions, point)
    }
    dimensions
}

fn perceive(dimensions: &mut Vec<Dimension>, point: C64) {
    let mut spectrum = Spectrum { point, length: 1 };
    for dimension in dimensions.iter_mut() {
        match dimension.perceive(spectrum) {
            Some(result) => spectrum = result,
            None => break
        }
    }
}

pub struct Spectrum {
    pub point: C64,
    pub length: usize,
}

pub type Label = String;

struct Stats {
    sample_mean: C64,
    sample_variance: C64,
    prior_mean: C64,
    prior_variance: C64,
}

struct Location {
    centroid: C64,
    radius: C64,
}

pub struct Dimension {
    level: u16,
    radius_scale: u16,
    resolution: u16,
    stats: HashMap<Label, Stats>,
    locations: HashMap<Label, Location>,
    unigram: HashMap<Label, usize>,
    bigram: HashMap<Label, HashMap<Label, usize>>,
    total: usize,
    prev: Label,
    ongoing: Vec<C64>,
    lengths: Vec<usize>,
    current: Vec<Label>,
    segments: Vec<Vec<Label>>,
    relative_lengths: Vec<Vec<usize>>,
}

impl Dimension {
    pub fn new(level: u16, radius_scale: u16, resolution: u16) -> Dimension {
        Dimension {
            level,
            radius_scale,
            resolution,
            stats: HashMap::new(),
            locations: HashMap::new(),
            unigram: HashMap::new(),
            bigram: HashMap::new(),
            total: 0,
            prev: Label::new(),
            ongoing: Vec::new(),
            lengths: Vec::new(),
            current: Vec::new(),
            segments: Vec::new(),
            relative_lengths: Vec::new(),
        }
    }

    pub fn perceive(&mut self, spectrum: Spectrum) -> Option<Spectrum> {
        let category = categorization::categorize(self, &spectrum);
        self.update(&category, &spectrum);
        if segmentation::segment(self, &category) {
            let superior = abstraction::transform(&self.ongoing);
            return Some(superior)
        }
        self.prev = category;
        None
    }

    pub fn update(&mut self, category: &String, spectrum: &Spectrum) {}


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let config = Config::default()?;
        let signal = vec![C64::new(1.0, 1.0); 100];
        let dimensions = process(&config, signal);
        Ok(())
    }
}
