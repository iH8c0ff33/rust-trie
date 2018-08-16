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
            _ => Trie::Node(head[0], false, vec![Self::new(tail.to_vec())]),
        }
    }

    pub fn first_char(&self) -> char {
        match self {
            Trie::Leaf(a) => *a,
            Trie::Node(a, _, _) => *a,
        }
    }

    pub fn insert(&mut self, word: Vec<char>) -> Result<(), &str> {
        let (head, tail) = word.split_at(1);
        if head.len() < 1 {
            return Err("empty word");
        }
        if head[0] != self.first_char() {
            return Err("char mismatch");
        }

        match tail.len() {
            0 => match self {
                Trie::Leaf(_) => Ok(()),
                Trie::Node(_, boundary, _) => {
                    *boundary = true;
                    Ok(())
                }
            },
            _ => match self {
                Trie::Leaf(_) => {
                    *self = Self::new(tail.to_vec());
                    Ok(())
                }
                Trie::Node(_, _, rest) => {
                    if let Some(position) =
                        rest.iter().position(|child| child.first_char() == tail[0])
                    {
                        return rest[position].insert(tail.to_vec());
                    }

                    rest.push(Trie::new(tail.to_vec()));
                    Ok(())
                }
            },
        }
    }

    pub fn contains(&self, word: Vec<char>) -> bool {
        if let Some((head, tail)) = word.split_first() {
            match tail.len() {
                0 => if let Trie::Leaf(c) = self {
                    c == head
                } else {
                    false
                },
                _ => if let Trie::Node(c, _, children) = self {
                    if c == head {
                        if let Some(child) = children.find_contains_char(&tail[0]) {
                            child.contains(tail.to_vec())
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                },
            }
        } else {
            false
        }
    }
}

pub trait FindChild {
    fn find_contains_char(&self, c: &char) -> Option<&Trie>;
}

impl FindChild for Vec<Trie> {
    fn find_contains_char(&self, c: &char) -> Option<&Trie> {
        self.iter().find(|child| child.first_char() == *c)
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
    fn trie_first_char() {
        let trie = Trie::new("daw".chars().collect());
        assert_eq!(trie.first_char(), 'd');

        let trie = Trie::new("hello".chars().collect());
        assert_eq!(trie.first_char(), 'h');
    }

    #[test]
    fn trie_insert_word() {
        let mut trie = Trie::new("daw".chars().collect());

        assert!(trie.insert("dat".chars().collect()).is_ok());
        assert!(trie.insert("dew".chars().collect()).is_ok());

        assert_eq!(
            trie,
            Node(
                'd',
                false,
                vec![
                    Node('a', false, vec![Leaf('w'), Leaf('t')]),
                    Node('e', false, vec![Leaf('w')]),
                ],
            )
        )
    }

    #[test]
    fn trie_find_child_containing_char() {
        let mut trie = Trie::new("daw".chars().collect());
        trie.insert("dat".chars().collect()).unwrap();
        trie.insert("dew".chars().collect()).unwrap();

        if let Node(_, _, children) = &trie {
            assert_eq!(
                *children.find_contains_char(&'a').unwrap(),
                Node('a', false, vec![Leaf('w'), Leaf('t')])
            );
            assert!(children.find_contains_char(&'b').is_none());
        };
    }

    #[test]
    fn trie_contains_word() {
        let mut trie = Trie::new("daw".chars().collect());
        trie.insert("dat".chars().collect()).unwrap();
        trie.insert("dew".chars().collect()).unwrap();

        assert!(trie.contains("dew".chars().collect()));
        assert!(!trie.contains("dan".chars().collect()));
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
