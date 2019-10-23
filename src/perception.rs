use crate::abstraction::C64;
use crate::Config;

use std::collections::HashMap;

pub fn perceive(config: &Config, signal: Vec<C64>) -> Vec<Dimension> {
    vec![Dimension::new(0, config.radius_scale, config.resolution)]
}

type Label = String;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
