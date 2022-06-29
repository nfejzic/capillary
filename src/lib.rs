struct Dictionary;

struct Node;

impl Dictionary {
    pub fn new() -> Self {
        Self
    }

    pub fn len(&self) -> usize {
        0
    }

    pub fn insert(&mut self, word: &str, substitution: &str) {
        todo!()
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
}
