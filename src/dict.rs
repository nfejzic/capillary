use petgraph::{graph::NodeIndex, stable_graph::StableGraph, visit::EdgeRef, Directed};

use crate::InvalidKeyPartErr;

/// Data structure for storing key-value pairs, with partial key lookup feature.
#[derive(Debug)]
pub struct Dictionary<K, V>
where
    K: PartialEq,
{
    pub(crate) graph: StableGraph<Option<V>, K, Directed, usize>,
    pub(crate) root: NodeIndex<usize>,
}

#[derive(Debug)]
pub struct Lookup<'a, K, V>
where
    K: PartialEq,
{
    dict: &'a Dictionary<K, V>,
    curr_node: NodeIndex<usize>,
}

impl<K, V> Default for Dictionary<K, V>
where
    K: PartialEq,
{
    fn default() -> Self {
        let mut graph = StableGraph::<Option<V>, K, Directed, usize>::default();
        let root = graph.add_node(None);

        Self { graph, root }
    }
}

impl<K, V> Dictionary<K, V>
where
    K: PartialEq,
{
    /// Creates a new empty `Dictionary`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks whether this `Dictionary` is empty - contains no key-value pairs.
    pub fn is_empty(&self) -> bool {
        // root node is always present, but dictionary is empty at that point
        self.graph.node_count() <= 1
    }

    /// Returns the number of key-value pairs contained in this `Dictionary`.
    pub fn len(&self) -> usize {
        self.graph
            .node_indices()
            .filter(|index| self.graph[*index].is_some())
            .count()
    }

    /// Inserts a key-value pair into the `Dictionary`.
    pub fn insert<W>(&mut self, key: W, value: V)
    where
        W: IntoIterator<Item = K>,
    {
        let mut iter = key.into_iter().peekable();

        let mut curr_node = self.root;

        while let Some(key_part) = iter.next() {
            match self
                .graph
                .edges(curr_node)
                .find(|edge| edge.weight() == &key_part)
            {
                Some(edge) => {
                    curr_node = edge.target();
                }
                None => match iter.peek() {
                    Some(_) => {
                        let new_node = self.graph.add_node(None);
                        self.graph.add_edge(curr_node, new_node, key_part);
                        curr_node = new_node;
                    }
                    None => {
                        // this node contains the value
                        let node_with_val = self.graph.add_node(Some(value));
                        self.graph.add_edge(curr_node, node_with_val, key_part);
                        break;
                    }
                },
            }
        }
    }

    /// Returns reference to the value paired with the given key. Returns None if there is no value
    /// stored with the given key.
    pub fn get<W>(&self, key: W) -> Option<&V>
    where
        W: IntoIterator<Item = K>,
    {
        let node = self.find_node(key)?;

        self.graph.node_weight(node)?.as_ref()
    }

    /// Returns mutable reference to the value paired with the given key. Returns None if there is
    /// no value stored with the given key.
    pub fn get_mut<W>(&mut self, key: W) -> Option<&mut V>
    where
        W: IntoIterator<Item = K>,
    {
        let node = self.find_node(key)?;

        self.graph.node_weight_mut(node)?.as_mut()
    }

    fn find_node<W>(&self, key: W) -> Option<NodeIndex<usize>>
    where
        W: IntoIterator<Item = K>,
    {
        let mut iter = key.into_iter().peekable();

        let mut curr_node = self.root;

        while let Some(key_part) = iter.next() {
            match self
                .graph
                .edges(curr_node)
                .find(|edge| edge.weight() == &key_part)
            {
                Some(edge) => {
                    curr_node = edge.target();

                    if iter.peek().is_none() {
                        return Some(curr_node);
                    }
                }
                None => return None,
            }
        }

        None
    }

    /// Creates a [`Lookup`] into the [`Dictionary`] used for searching inside of the dictionary.
    pub fn lookup(&self) -> Lookup<'_, K, V> {
        Lookup {
            dict: self,
            curr_node: self.root,
        }
    }
}

impl<K, V> Lookup<'_, K, V>
where
    K: PartialEq,
{
    /// Moves along some key path if and only if the given key part is reachable from current
    /// state. If the given key part is not reachable from current state, the state of `Lookup`
    /// is reset to the default state, and `Err` is returned.
    pub fn partial_search(&mut self, key_part: &K) -> Result<(), InvalidKeyPartErr> {
        match self
            .dict
            .graph
            .edges(self.curr_node)
            .find(|edge| edge.weight() == key_part)
        {
            Some(edge) => self.curr_node = edge.target(),
            None => {
                self.curr_node = self.dict.root;
                return Err(InvalidKeyPartErr);
            }
        }

        Ok(())
    }

    /// Tries to retrieve value associated with key constructed from key parts so far provided with
    /// [`Lookup::partial_search`].
    pub fn try_resolve(&self) -> Option<&V> {
        self.dict.graph.node_weight(self.curr_node)?.as_ref()
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
        self.curr_node = self.dict.root;
    }
}

#[cfg(test)]
mod tests {
    use super::Dictionary;

    #[test]
    fn insert() {
        let mut dict = Dictionary::new();

        dict.insert(":D".chars(), "Hello");

        assert_eq!(dict.len(), 1);

        let val = dict.get(":D".chars());

        assert_eq!(val, Some(&"Hello"));
    }

    #[test]
    fn partial_find() {
        let mut dict = Dictionary::new();
        dict.insert(":D".chars(), "Hello");

        let mut lookup = dict.lookup();

        let _ = lookup.partial_search(&':');
        let _ = lookup.partial_search(&'D');

        let val = lookup.try_resolve();

        assert_eq!(val, Some(&"Hello"));
    }

    #[test]
    fn multiple_vals() {
        let mut dict = Dictionary::new();
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

        assert_eq!(val1, Some(&"Hi there"));
        assert_eq!(val2, Some(&"Hello"));
    }

    #[test]
    fn mutate_node() {
        let mut dict = Dictionary::new();
        dict.insert(":D".chars(), String::from("Hi there"));

        let val = dict.get_mut(":D".chars()).unwrap();

        *val = String::from("Hi capillary!");

        let new_val = dict.get(":D".chars());

        assert_eq!(new_val, Some(&String::from("Hi capillary!")));
    }
}
