use num::complex::Complex64;
use std::collections::HashMap;

use crate::abstraction::Spectrum;
use crate::concept_symbol::{Concept, Label};
use crate::dimension::Dimension;
use crate::markov_model::UnigramModel;

pub fn categorize(
    concept: &Concept,
    concepts: &HashMap<Label, Concept>,
    unigram: &UnigramModel<Label>,
) -> Label {
    concepts
        .iter()
        .map(|(_, c)| c)
//        .filter(|c| norm(c.location.centroid - concept.location.centroid) <= c.location.radius)
        .min_by_key(|c| unigram.count(&c.label))
        .unwrap_or(concept)
        .label
}

fn norm(point: Complex64) -> f64 {
    point.norm()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
