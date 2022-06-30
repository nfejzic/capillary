use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

type NodesMap<K, V> = HashMap<K, Rc<RefCell<Node<K, V>>>>;

pub struct InvalidKeyPartErr;

#[derive(Debug, Clone)]
pub struct Dictionary<K, V>
where
    K: Hash + PartialEq + Eq,
{
    len: usize,
    depth: usize,
    nodes: NodesMap<K, V>,
    curr_node: Option<Rc<RefCell<Node<K, V>>>>,
}

#[derive(Debug, Clone)]
struct Node<K, V>
where
    K: Hash,
{
    data: Option<V>,
    nodes: NodesMap<K, V>,
}

impl<K, V> Default for Node<K, V>
where
    K: Hash,
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn insert<I>(&mut self, word: I, substitution: V)
    where
        I: IntoIterator<Item = K>,
    {
        let mut curr_node: Option<Rc<RefCell<Node<K, V>>>> = None;
        let mut iter = word.into_iter().peekable();

        while let Some(key_part) = iter.next() {
            match curr_node {
                Some(node) => {
                    let mut new_node = node.borrow_mut();

                    if iter.peek().is_none() {
                        self.len += 1;

                        let child = Rc::new(
                            Node {
                                data: Some(substitution),
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

    pub fn try_resolve_path(&self) -> Option<impl Deref<Target = V> + '_> {
        let curr_node = self.curr_node.as_ref()?;
        if curr_node.borrow().data.is_some() {
            Some(Ref::map(curr_node.borrow(), |node| {
                node.data.as_ref().unwrap()
            }))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Dictionary;

    #[test]
    fn insert() {
        let mut d = Dictionary::new();

        d.insert([":", "D"], "Hello");

        assert_eq!(d.len(), 1);
    }

    #[test]
    fn insert_multiple() {
        let mut d = Dictionary::new();

        d.insert([":", "D"], "Hello");
        d.insert([":", ")"], "There");

        assert_eq!(d.len(), 2);
    }

    #[test]
    fn find_by_partial_codes() {
        let mut d = Dictionary::new();

        d.insert([":", "D"], "Hello");

        let _ = d.partial_find(&":");
        let _ = d.partial_find(&"D");

        let val = d.try_resolve_path();

        assert_eq!(val.as_deref(), Some(&"Hello"));
    }
}
