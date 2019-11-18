use crate::concept_symbol::Label;
use crate::markov_model::{BigramModel, UnigramModel};
use std::collections::HashMap;

/// Determines whether to segment at the current position
///
/// # Arguments
/// * `unigram` - unigram model for this sequence
/// * `previous` - label of the symbol before the current symbol
/// * `current` - label of the current symbol
///
pub fn segment(
    unigram: &UnigramModel<Label>,
    previous: &Label,
    current: &Label
) -> bool {
    // Equivalent to comparing information content i.e. -log2(count/total)
    unigram.count(previous) > unigram.count(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
