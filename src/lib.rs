#[derive(Debug)]
pub enum Trie {
    Leaf(char),
    Node(char, bool, Vec<Trie>),
}

impl Trie {
    pub fn new(word: Vec<char>) -> Self {
        let (head, tail) = word.split_at(1);
        match tail.len() {
            0 => Trie::Leaf(head[0]),
            _ => Trie::Node(head[0], false, vec![Trie::new(tail.to_vec())]),
        }
    }
}

impl PartialEq for Trie {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Trie::Leaf(a) => match other {
                Trie::Leaf(b) => a == b,
                _ => false,
            },
            Trie::Node(a, _, rest) => match other {
                Trie::Node(b, _, other) => a == b && rest == other,
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trie::{Leaf, Node};
    use super::*;

    #[test]
    fn create_trie_word() {
        let trie = Trie::new("hello".chars().collect());
        assert_eq!(
            trie,
            Node(
                'h',
                false,
                vec![Node(
                    'e',
                    false,
                    vec![Node('l', false, vec![Node('l', false, vec![Leaf('o')])])]
                )]
            )
        )
    }

    #[test]
    fn trie_partial_eq() {
        let a = Node(
            'd',
            false,
            vec![
                Node('e', false, vec![Leaf('w')]),
                Node('a', false, vec![Leaf('w'), Leaf('t')]),
            ],
        );

        let b = Node(
            'd',
            false,
            vec![
                Node('e', false, vec![Leaf('w')]),
                Node('a', false, vec![Leaf('w'), Leaf('t')]),
            ],
        );

        assert_eq!(a, b);
    }
}
