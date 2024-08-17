use std::{collections::HashMap, hash::Hash};

use crate::InvalidKeyPartErr;

/// Data structure for storing key-value pairs, with partial key lookup feature.
#[derive(Debug)]
pub struct VecDict<K, V>
where
    K: Eq + Hash,
{
    pub(crate) indices: HashMap<K, usize>,
    pub(crate) nodes: Vec<Node<K, V>>,
    len: usize,
}

#[derive(Debug)]
pub(crate) struct Node<K, V>
where
    K: Eq + Hash,
{
    value: Option<V>,
    indices: HashMap<K, usize>,
}

#[derive(Debug)]
pub struct Lookup<'a, K, V>
where
    K: Eq + Hash,
{
    dict: &'a VecDict<K, V>,
    curr_index: Option<usize>,
}

impl<K, V> Default for Node<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            value: Default::default(),
            indices: Default::default(),
        }
    }
}

impl<K, V> Default for VecDict<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            indices: Default::default(),
            nodes: Default::default(),
            len: Default::default(),
        }
    }
}

impl<K, V> VecDict<K, V>
where
    K: Eq + Hash + std::fmt::Debug,
{
    /// Creates a new empty `Dictionary`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks whether this `Dictionary` is empty - contains no key-value pairs.
    pub fn is_empty(&self) -> bool {
        true
    }

    /// Returns the number of key-value pairs contained in this `Dictionary`.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Inserts a key-value pair into the `Dictionary`.
    pub fn insert<W>(&mut self, key: W, value: V)
    where
        W: IntoIterator<Item = K>,
    {
        let mut curr_idx: Option<usize> = None;
        for key_part in key.into_iter() {
            match curr_idx {
                Some(idx) => match self.nodes[idx].indices.get(&key_part) {
                    Some(idx) => curr_idx = Some(*idx),
                    None => {
                        self.nodes.push(Node::default());
                        let new_idx = self.nodes.len() - 1;
                        self.nodes[idx].indices.insert(key_part, new_idx);
                        curr_idx = Some(new_idx);
                    }
                },
                None => match self.indices.get(&key_part) {
                    Some(idx) => curr_idx = Some(*idx),
                    None => {
                        self.nodes.push(Node::default());
                        self.indices.insert(key_part, self.nodes.len() - 1);
                        curr_idx = Some(self.nodes.len() - 1);
                    }
                },
            };
        }

        if let Some(idx) = curr_idx {
            self.nodes[idx].value = Some(value);
            self.len += 1;
        }
    }

    /// Returns reference to the value paired with the given key. Returns None if there is no value
    /// stored with the given key.
    pub fn get<W>(&self, key: W) -> Option<&V>
    where
        W: IntoIterator<Item = K>,
    {
        let node_idx = self.find_node(key)?;
        self.nodes.get(node_idx)?.value.as_ref()
    }

    /// Returns mutable reference to the value paired with the given key. Returns None if there is
    /// no value stored with the given key.
    pub fn get_mut<W>(&mut self, key: W) -> Option<&mut V>
    where
        W: IntoIterator<Item = K>,
    {
        let node_idx = self.find_node(key)?;
        self.nodes.get_mut(node_idx)?.value.as_mut()
    }

    fn find_node<W>(&self, key: W) -> Option<usize>
    where
        W: IntoIterator<Item = K>,
    {
        let mut curr_index: Option<usize> = None;

        for key_part in key.into_iter() {
            match curr_index {
                Some(idx) => {
                    let node = self.nodes.get(idx)?;

                    curr_index = node.indices.get(&key_part).copied();
                }
                None => {
                    curr_index = self.indices.get(&key_part).copied();
                }
            }
        }

        curr_index
    }

    /// Creates a [`Lookup`] into the [`Dictionary`] used for searching inside of the dictionary.
    pub fn lookup(&self) -> Lookup<'_, K, V> {
        Lookup {
            dict: self,
            curr_index: None,
        }
    }
}

impl<K, V> Lookup<'_, K, V>
where
    K: Eq + Hash + std::fmt::Debug,
    V: std::fmt::Debug,
{
    /// Moves along some key path if and only if the given key part is reachable from current
    /// state. If the given key part is not reachable from current state, the state of `Lookup`
    /// is reset to the default state, and `Err` is returned.
    pub fn partial_search(&mut self, key_part: &K) -> Result<(), InvalidKeyPartErr> {
        match self.curr_index {
            Some(idx) => match self.dict.nodes.get(idx) {
                Some(node) if node.indices.contains_key(key_part) => {
                    self.curr_index = node.indices.get(key_part).copied();
                }
                _ => return Err(InvalidKeyPartErr),
            },
            None => {
                if self.dict.indices.contains_key(key_part) {
                    self.curr_index = self.dict.indices.get(key_part).copied();
                } else {
                    return Err(InvalidKeyPartErr);
                }
            }
        }

        Ok(())
    }

    /// Tries to retrieve value associated with key constructed from key parts so far provided with
    /// [`Lookup::partial_search`].
    pub fn try_resolve(&self) -> Option<&V> {
        self.dict.nodes.get(self.curr_index?)?.value.as_ref()
    }

    /// Returns reference to the value paired with the given key. Returns None if there is no value
    /// stored with the given key.
    pub fn get<W>(&self, key: W) -> Option<&V>
    where
        W: IntoIterator<Item = K>,
    {
        self.dict.get(key)
    }

    /// Resets the state of the lookup. Typically used after calling [`Lookup::partial_search`] and
    /// wanting to start the search over from clear slate.
    pub fn reset(&mut self) {
        self.curr_index = None;
    }
}

#[cfg(test)]
mod tests {
    use super::VecDict;

    #[test]
    fn insert() {
        let mut dict = VecDict::new();

        dict.insert(":D".chars(), "Hello");

        assert_eq!(dict.len(), 1);

        let val = dict.get(":D".chars());

        assert_eq!(val, Some(&"Hello"));
    }

    #[test]
    fn partial_find() {
        let mut dict = VecDict::new();
        dict.insert(":D".chars(), "Hello");

        let mut lookup = dict.lookup();

        let _ = lookup.partial_search(&':');
        let _ = lookup.partial_search(&'D');

        let val = lookup.try_resolve();

        assert_eq!(val, Some(&"Hello"));
    }

    #[test]
    fn multiple_vals() {
        let mut dict = VecDict::new();
        dict.insert(":D".chars(), "Hi there");
        dict.insert(":)".chars(), "Hello");

        let mut lookup = dict.lookup();

        let _ = lookup.partial_search(&':');
        let _ = lookup.partial_search(&'D');

        let val1 = lookup.try_resolve();

        let mut lookup = dict.lookup();

        let _ = lookup.partial_search(&':');
        let _ = lookup.partial_search(&')');

        let val2 = lookup.try_resolve();

        dbg!(&dict);

        assert_eq!(val1, Some(&"Hi there"));
        assert_eq!(val2, Some(&"Hello"));
    }

    #[test]
    fn mutate_node() {
        let mut dict = VecDict::new();
        dict.insert(":D".chars(), String::from("Hi there"));

        let val = dict.get_mut(":D".chars()).unwrap();

        *val = String::from("Hi capillary!");

        let new_val = dict.get(":D".chars());

        assert_eq!(new_val, Some(&String::from("Hi capillary!")));
    }
}
