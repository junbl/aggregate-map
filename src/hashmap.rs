//! Implementation of [`Map`] for a [`HashMap`].
use std::collections::HashMap;

use crate::Map;

impl<K, V, C> Map<K, V> for HashMap<K, C>
where
    K: Eq + std::hash::Hash,
    C: Default + Extend<V>,
{
    fn insert(&mut self, key: K, value: V) {
        self.entry(key).or_default().extend(std::iter::once(value));
    }
}
