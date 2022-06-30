use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    ops::Deref,
    rc::Rc,
};

type NodesMap = HashMap<String, Rc<RefCell<Node>>>;

pub struct InvalidKeyPartErr;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Dictionary {
    len: usize,
    depth: usize,
    path: String,
    nodes: NodesMap,
    curr_node: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Node {
    subst: Option<String>,
    nodes: NodesMap,
}

impl Dictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, word: &'static str, substitution: &str) {
        let mut curr_node: Option<Rc<RefCell<Node>>> = None;
        let mut iter = word.chars().peekable();

        while let Some(char) = iter.next() {
            match curr_node {
                Some(node) => {
                    let mut new_node = node.borrow_mut();
                    let created_node =
                        new_node.nodes.entry(String::from(char)).or_insert_with(|| {
                            if iter.peek().is_some() {
                                Default::default()
                            } else {
                                self.len += 1;

                                Rc::new(
                                    Node {
                                        subst: Some(String::from(substitution)),
                                        ..Default::default()
                                    }
                                    .into(),
                                )
                            }
                        });

                    curr_node = Some(Rc::clone(created_node));
                }
                None => {
                    curr_node = Some(Rc::clone(
                        self.nodes
                            .entry(String::from(char))
                            .or_insert_with(Default::default),
                    ));
                }
            }
        }
    }

    pub fn curr_depth(&self) -> usize {
        self.path.len()
    }

    pub fn partial_find(&mut self, code: &str) -> Result<(), InvalidKeyPartErr> {
        match self.curr_node.take() {
            Some(node) => match node.borrow().nodes.get(code) {
                Some(new_node) => self.curr_node = Some(Rc::clone(new_node)),
                None => return Err(InvalidKeyPartErr),
            },
            None => match self.nodes.get(code) {
                Some(new_node) => self.curr_node = Some(Rc::clone(new_node)),
                None => return Err(InvalidKeyPartErr),
            },
        }

        Ok(())
    }

    pub fn try_resolve_path(&self) -> Option<impl Deref<Target = str> + '_> {
        let curr_node = self.curr_node.as_ref()?;
        if curr_node.borrow().subst.is_some() {
            Some(Ref::map(curr_node.borrow(), |node| {
                node.subst.as_ref().unwrap().as_str()
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

        d.insert(":D", "Hello");

        assert_eq!(d.len(), 1);
    }

    #[test]
    fn insert_multiple() {
        let mut d = Dictionary::new();

        d.insert(":D", "Hello");
        d.insert(":)", "There");

        assert_eq!(d.len(), 2);
    }

    #[test]
    fn find_by_partial_codes() {
        let mut d = Dictionary::new();

        d.insert(":D", "Hello");

        let _ = d.partial_find(":");
        let _ = d.partial_find("D");

        let val = d.try_resolve_path();

        assert_eq!(val.as_deref(), Some("Hello"));
    }
}
