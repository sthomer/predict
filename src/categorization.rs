use num::complex::Complex64;
use std::collections::HashMap;

use crate::abstraction::Spectrum;
use crate::concept_symbol::{Concept, Label};
use crate::dimension::Dimension;
use crate::markov_model::UnigramModel;

/// Returns the label of the category to which the given concept belongs.
/// If concept can belong to multiple categories, it is placed in the most rare,
/// i.e. the one with the highest information content / entropy.
///
/// # Arguments
/// * `concept` - concept to categorize
/// * `concepts` - map of categories to compare input concept to
/// * `unigram` - unigram model to determine tie-breaks
///
pub fn categorize(
    concept: &Concept,
    concepts: &HashMap<Label, Concept>,
    unigram: &UnigramModel<Label>,
) -> Label {
    concepts.iter().map(|(_, concept)| concept)
//        .filter(|category| member(category, concept))
        .min_by_key(|concept| unigram.count(&concept.label))
        .unwrap_or(concept)
        .label
}

/// Returns whether the concept is a member of the category
///
/// # Arguments
/// * `category` - category to determine membership of
/// * `concept` - concept to determine if a member
///
fn member(category: &Concept, target: &Concept) -> bool {
//    let distance = norm(&category.location.centroid - &target.location.centroid);
//    distance <= category.location.radius
    unimplemented!()
}

/// Returns the norm (length) of the given point.
/// # Arguments
/// * `point` - vector representation
///
fn norm(point: Complex64) -> f64 {
    point.norm()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
