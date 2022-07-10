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

use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

type NodesMap<K, V> = HashMap<K, Rc<RefCell<Node<K, V>>>>;

/// Error indicating that some key part lead to invalid [`Dictionary`] state, thus resetting the
/// [`Dictionary`] to the default state.
pub struct InvalidKeyPartErr;

/// Data structure for storing key-value pairs, with partial key lookup feature.
#[derive(Debug, Clone)]
pub struct Dictionary<K, V>
where
    K: Hash + Eq,
{
    len: usize,
    depth: usize,
    nodes: NodesMap<K, V>,
    curr_node: Option<Rc<RefCell<Node<K, V>>>>,
}

#[derive(Debug, Clone)]
struct Node<K, V>
where
    K: Eq + Hash,
{
    data: Option<V>,
    nodes: NodesMap<K, V>,
}

impl<K, V> Default for Node<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            data: None,
            nodes: HashMap::new(),
        }
    }
}

impl<K, V> Default for Dictionary<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self {
            len: Default::default(),
            depth: Default::default(),
            nodes: Default::default(),
            curr_node: Default::default(),
        }
    }
}

impl<K, V> Dictionary<K, V>
where
    K: Hash + PartialEq + Eq,
{
    /// Creates a new empty `Dictionary`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks whether this `Dictionary` is empty - contains no key-value pairs.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of key-value pairs contained in this `Dictionary`.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the current depth along the key path. Returns 0 in default state.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Inserts a key-value pair into the `Dictionary`.
    pub fn insert<W>(&mut self, key: W, value: V)
    where
        W: IntoIterator<Item = K>,
    {
        let mut curr_node: Option<Rc<RefCell<Node<K, V>>>> = None;
        let mut iter = key.into_iter().peekable();

        while let Some(key_part) = iter.next() {
            match curr_node {
                Some(node) => {
                    let mut new_node = node.borrow_mut();

                    if iter.peek().is_none() {
                        self.len += 1;

                        let child = Rc::new(
                            Node {
                                data: Some(value),
                                ..Default::default()
                            }
                            .into(),
                        );

                        new_node.nodes.entry(key_part).or_insert(child);

                        break;
                    }

                    let child = Default::default();
                    let created_node = new_node.nodes.entry(key_part).or_insert(child);
                    curr_node = Some(Rc::clone(created_node));
                }
                None => {
                    curr_node = Some(Rc::clone(
                        self.nodes.entry(key_part).or_insert_with(Default::default),
                    ));
                }
            }
        }
    }

    /// Moves along some key path if and only if the given key part is reachable from current
    /// state. If the given key part is not reachable from current state, the state of `Dictionary`
    /// is reset to the default state, and `Err` is returned.
    pub fn partial_find(&mut self, code: &K) -> Result<(), InvalidKeyPartErr> {
        match self.curr_node.take() {
            Some(node) => match node.borrow().nodes.get(code) {
                Some(new_node) => {
                    self.depth += 1;
                    self.curr_node = Some(Rc::clone(new_node))
                }
                None => {
                    self.depth = 0;
                    return Err(InvalidKeyPartErr);
                }
            },
            None => match self.nodes.get(code) {
                Some(new_node) => {
                    self.depth += 1;
                    self.curr_node = Some(Rc::clone(new_node))
                }
                None => {
                    self.depth = 0;
                    return Err(InvalidKeyPartErr);
                }
            },
        }

        Ok(())
    }

    /// Tries to retrieve value associated with key constructed from key parts so far provided with
    /// [`Dictionary::partial_find`].
    pub fn try_resolve(&self) -> Option<impl Deref<Target = V> + '_> {
        let curr_node = self.curr_node.as_ref()?;
        if curr_node.borrow().data.is_some() {
            Some(Ref::map(curr_node.borrow(), |node| {
                node.data.as_ref().unwrap()
            }))
        } else {
            None
        }
    }

    /// Returns the value paired with the given key. Returns None if such value does not exist.
    ///
    /// Does not return reference to the value, clones it instead. Returning the reference involves
    /// mutating internal state. In order to avoid that requirement, value is cloned so that the
    /// function can be called on immutable references too.
    pub fn get<W>(&self, key: W) -> Option<V>
    where
        W: IntoIterator<Item = K>,
        V: Clone,
    {
        let mut iter = key.into_iter().peekable();
        let first_key_part = iter.next()?;

        let mut curr_node: Rc<RefCell<_>> = Rc::clone(self.nodes.get(&first_key_part)?);

        for ref key_part in iter {
            let new_node = Rc::clone(curr_node.borrow().nodes.get(key_part)?);
            curr_node = new_node;
        }

        let node = curr_node.borrow();
        node.data.as_ref().cloned()
    }

    /// Returns reference to the value paired with the given key. Returns None if such value does
    /// not exist.
    ///
    /// Resets the current search state of dictionary. This means that any started partial search
    /// will be reset.
    pub fn get_ref<W>(&mut self, key: W) -> Option<impl Deref<Target = V> + '_>
    where
        W: IntoIterator<Item = K>,
    {
        let temp = self.curr_node.take();

        let iter = key.into_iter();

        for ref key_part in iter {
            if self.partial_find(key_part).is_err() {
                self.curr_node = temp;
                return None;
            }
        }

        self.try_resolve()
    }
}

impl<KeyParts, K, V> FromIterator<(KeyParts, V)> for Dictionary<K, V>
where
    KeyParts: IntoIterator<Item = K>,
    K: Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = (KeyParts, V)>>(iter: I) -> Self {
        let mut dictionary = Self::default();

        for (key, value) in iter {
            dictionary.insert(key, value);
        }

        dictionary
    }
}

#[cfg(test)]
mod tests {
    use crate::Dictionary;

    #[test]
    fn insert_slice() {
        let mut d = Dictionary::new();

        let (key, value) = (&[":", "D"], "Hello");

        d.insert(key, value);
        assert_eq!(d.len(), 1);

        assert_eq!(d.get(&[":", "D"]), Some("Hello"));
    }

    #[test]
    fn string_val() {
        let mut d = Dictionary::new();

        let key = ":D";

        d.insert(key.chars(), String::from("Hello"));

        assert_eq!(d.get(key.chars()), Some(String::from("Hello")));
    }

    #[test]
    fn insert() {
        let mut d = Dictionary::new();

        d.insert(":D".chars(), "Hello");

        assert_eq!(d.len(), 1);
    }

    #[test]
    fn insert_multiple() {
        let mut d = Dictionary::new();

        d.insert(":D".chars(), "Hello");
        d.insert(":)".chars(), "There");

        assert_eq!(d.len(), 2);
    }

    #[test]
    fn find_by_partial_codes() {
        let mut d = Dictionary::new();

        d.insert(":D".chars(), "Hello");

        let _ = d.partial_find(&':');
        let _ = d.partial_find(&'D');

        let val = d.try_resolve();

        assert_eq!(val.as_deref(), Some(&"Hello"));
    }

    #[test]
    fn collect() {
        let input = [(":D".chars(), "Hello")];

        let mut d: Dictionary<_, _> = input.into_iter().collect();

        assert_eq!(d.len(), 1);

        let _ = d.partial_find(&':');
        let _ = d.partial_find(&'D');

        let val = d.try_resolve();

        assert_eq!(val.as_deref(), Some(&"Hello"));
    }

    #[test]
    fn insert_while_in_find() {
        let mut d = Dictionary::new();

        d.insert(":D".chars(), "Hello");

        let _ = d.partial_find(&':');

        d.insert(":)".chars(), "Aha");

        assert_eq!(d.len(), 2);
    }

    #[test]
    fn insert_while_ref_to_value() {
        let mut d = Dictionary::new();

        d.insert(":D".chars(), "Hello");

        let _ = d.partial_find(&':');
        let _ = d.partial_find(&'D');

        d.insert(":)".chars(), "Hi there");
    }
}
