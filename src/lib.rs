//! capillary introduces a [`Dictionary`] data structure. It's used for cases where search by
//! partial key is needed.
//!
//! In particular useful when performing some kind of find and replace in some data.
//!
//! i.e. one might want to perform find and replace in a string. With `capillary::Dictionary` it is
//! possible to keep starting a search, and `Dictionary` will be in __default__ state until some
//! character is part of a valid key. As long as the following characters are part of a valid key,
//! the state of `Dictionary` will be set to some part of the `key` towards the `value`. It is then
//! possible to test if some `value` is reached, and return it as soon as it gets hit.

pub mod dict;

pub use dict::*;

/// Error indicating that some key part lead to invalid [`Dictionary`] state, thus resetting the
/// [`Dictionary`] to the default state.
pub struct InvalidKeyPartErr;

impl<KeyParts, K, V> FromIterator<(KeyParts, V)> for Dictionary<K, V>
where
    KeyParts: IntoIterator<Item = K>,
    K: PartialEq,
{
    fn from_iter<I: IntoIterator<Item = (KeyParts, V)>>(iter: I) -> Self {
        let mut dictionary = Self::default();

        for (key, value) in iter {
            dictionary.insert(key, value);
        }

        dictionary
    }
}
