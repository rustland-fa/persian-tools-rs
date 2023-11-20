use std::ops::Deref;

/// A Fixed map that can hold a fixed amount of value inside it.
pub struct FixedMap<K: 'static, V: 'static>(pub(crate) &'static [(K, V)]);

impl<K, V> FixedMap<K, V>
where
    K: PartialEq,
{
    /// get the value of given key
    pub fn get<T: PartialEq<K>>(&self, key: &T) -> Option<&V> {
        self.0.iter().find_map(|(k, v)| (key == k).then_some(v))
    }
}

impl<K, V> Deref for FixedMap<K, V> {
    type Target = [(K, V)];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
