use Trie;

impl Trie {
    /// Creates a new empty node
    pub fn empty(c: char) -> Self {
        Trie::Node(c, false, vec![])
    }

    pub fn from_word(word: Vec<char>) -> Self {
        let (head, tail) = word
            .split_first()
            .expect("trie_from_word can't be called with empty word");

        if tail.len() == 0 {
            Trie::Leaf(*head)
        } else {
            Trie::Node(*head, false, vec![Trie::from_word(tail.to_vec())])
        }
    }

    /// Get the value stored at this node
    pub fn value(&self) -> &char {
        trie_value(self)
    }

    /// Get a mutable reference to the value stored at this node
    pub fn value_mut(&mut self) -> &mut char {
        trie_value_mut(self)
    }

    /// Checks whether this node is a word boundary
    pub fn is_boundary(&self) -> bool {
        match self {
            Trie::Leaf(_) => true,
            Trie::Node(_, boundary, _) => *boundary,
        }
    }

    /// Compute the number of chars stored in this node's subtrie
    pub fn compute_size(&self) -> usize {
        let mut size = 1;

        if let Trie::Node(_, _, children) = self {
            for child in children {
                size += child.compute_size();
            }
        };

        size
    }
}

macro_rules! value_fn {
    (name: $name:ident, type: $type:ty, ret: $ret_t:ty) => {
        pub fn $name(trie: $type) -> $ret_t {
            match trie {
                Trie::Leaf(c) => c,
                Trie::Node(c, _, _) => c,
            }
        }
    };
}

value_fn!(name: trie_value, type: &Trie, ret: &char);
value_fn!(name: trie_value_mut, type: &mut Trie, ret: &mut char);

impl PartialEq for Trie {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Trie::Leaf(a) => if let Trie::Leaf(b) = other {
                a == b
            } else {
                false
            },
            Trie::Node(a_value, a_boundary, a_children) => {
                if let Trie::Node(b_value, b_boundary, b_children) = other {
                    a_value == b_value && a_boundary == b_boundary && a_children == b_children
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trie::{Leaf, Node};
    use super::*;

    #[test]
    fn trie_node_empty() {
        let trie = Trie::empty('a');

        assert_eq!(trie, Node('a', false, vec![]));
    }

    #[test]
    fn trie_node_value() {
        let trie = Leaf('a');

        assert_eq!(trie.value(), &'a');

        let trie = Leaf('b');

        assert_eq!(trie.value(), &'b');
    }

    #[test]
    fn trie_node_value_mut() {
        let mut trie = Leaf('a');
        *trie.value_mut() = 'b';

        assert_eq!(trie.value(), &'b');
    }

    #[test]
    fn trie_node_is_boundary() {
        let trie = Leaf('a');
        assert!(trie.is_boundary());

        let trie = Trie::empty('b');
        assert!(!trie.is_boundary());

        let trie = Node('c', true, vec![]);
        assert!(trie.is_boundary());
    }

    #[test]
    fn trie_node_compute_size() {
        let trie = Node(
            'a',
            false,
            vec![Leaf('b'), Node('c', false, vec![Leaf('d'), Leaf('e')])],
        );
        assert_eq!(trie.compute_size(), 5);
    }
}
