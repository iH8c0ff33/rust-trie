use Trie;

impl Trie {
    /// Creates a new empty node
    pub fn empty(c: char) -> Self {
        Trie {
            key: c,
            boundary: true,
            children: vec![],
        }
    }

    pub fn from_word(word: Vec<char>) -> Self {
        let (head, tail) = word
            .split_first()
            .expect("trie_from_word can't be called with empty word");

        Trie {
            key: *head,
            boundary: tail.len() == 0,
            children: if tail.len() == 0 {
                vec![]
            } else {
                vec![Trie::from_word(tail.to_vec())]
            },
        }
    }

    /// Compute the number of chars stored in this node's subtrie
    pub fn compute_size(&self) -> usize {
        let mut size = 1;

        for child in &self.children {
            size += child.compute_size();
        }

        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_node_empty() {
        let trie = Trie::empty('a');

        assert_eq!(
            trie,
            Trie {
                key: 'a',
                boundary: true,
                children: vec![]
            }
        );
    }

    #[test]
    fn trie_node_compute_size() {
        let mut trie = Trie::from_word(vec!['a', 'b', 'c']);
        trie.insert(vec!['a', 'b', 'd']).unwrap();
        trie.insert(vec!['a', 'd', 'd']).unwrap();
        assert_eq!(trie.compute_size(), 6);
    }
}
