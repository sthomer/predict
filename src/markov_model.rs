use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::hash::Hash;

pub struct UnigramModel<K>
where
    K: Eq + Hash + Copy,
{
    unigram: HashMap<K, usize>,
    total: usize,
}

impl<K> UnigramModel<K>
where
    K: Eq + Hash + Copy,
{
    pub fn new() -> UnigramModel<K> {
        UnigramModel {
            unigram: HashMap::new(),
            total: 0,
        }
    }

    pub fn increment(&mut self, key: &K) {
        self.total += 1;
        *self.unigram.entry(*key).or_insert(0) += 1;
    }

    pub fn count(&self, key: &K) -> usize {
        match self.unigram.get(key) {
            Some(count) => *count,
            None => 0,
        }
    }
}

pub struct BigramModel<K>
where
    K: Eq + Hash + Copy,
{
    bigram: HashMap<K, UnigramModel<K>>,
    total: usize,
}

impl<K> BigramModel<K>
where
    K: Eq + Hash + Copy,
{
    pub fn new() -> BigramModel<K> {
        BigramModel {
            bigram: HashMap::new(),
            total: 0,
        }
    }

    pub fn increment(&mut self, first: &K, second: &K) {
        self.total += 1;
        self.bigram
            .entry(*first)
            .or_insert(UnigramModel::new())
            .increment(second);
    }

    pub fn count(&self, key: &K) -> usize {
        match self.bigram.get(key) {
            Some(unigram) => unigram.total,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
