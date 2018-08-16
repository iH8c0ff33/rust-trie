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
    use super::*;

    #[test]
    fn create_trie_word() {
        let trie = Trie::new("hello".chars().collect());
        assert_eq!(
            trie,
            Trie::Node(
                'h',
                false,
                vec![Trie::Node(
                    'e',
                    false,
                    vec![Trie::Node(
                        'l',
                        false,
                        vec![Trie::Node('l', false, vec![Trie::Leaf('o')])]
                    )]
                )]
            )
        )
    }
}
