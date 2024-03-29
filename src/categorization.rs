use num::complex::Complex64;
use std::collections::HashMap;
use crate::concept_symbol::{Concept, Label};
use crate::markov_model::UnigramModel;
use ndarray_linalg::norm::Norm;

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
        .filter(|category| member(category, concept))
        .min_by_key(|concept| unigram.count(&concept.label))
        .unwrap_or(concept)
        .label
}

/// Decides whether a concept is a member of a category
///
/// # Arguments
/// * `category` - category to determine membership of
/// * `concept` - concept to determine membership
///
fn member(category: &Concept, target: &Concept) -> bool {
    let distance = (&category.location.centroid - &target.location.centroid).norm();
    distance <= category.location.radius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
