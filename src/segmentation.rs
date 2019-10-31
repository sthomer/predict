use crate::perception::{UnigramModel, Label};
use std::collections::HashMap;

pub fn segment(unigram: &UnigramModel, previous: &Label, current: &Label) -> bool {
    // Equivalent to comparing information content i.e. -log2(count/total)
    unigram.count(previous) > unigram.count(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
