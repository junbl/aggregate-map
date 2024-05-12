//! Implementation of [`Map`] for a [`BTreeMap`].
use std::collections::BTreeMap;

use crate::Map;

impl<K, V, C> Map<K, V> for BTreeMap<K, C>
where
    K: Eq + Ord + std::hash::Hash,
    C: Default + Extend<V>,
{
    fn insert(&mut self, key: K, value: V) {
        self.entry(key).or_default().extend(std::iter::once(value));
    }
}
