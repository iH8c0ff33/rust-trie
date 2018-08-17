use Trie;

impl Trie {
    pub fn get(&self, prefix: Vec<char>) -> Option<&Self> {
        trie_get(self, prefix)
    }

    pub fn get_mut(&mut self, prefix: Vec<char>) -> Option<&mut Self> {
        trie_get_mut(self, prefix)
    }

    pub fn insert(&mut self, word: Vec<char>) -> Option<bool> {
        let (head, tail) = word
            .split_first()
            .expect("trie_insert can't be called with empty words");
        if head != self.value() {
            return None;
        }

        if tail.len() == 0 {
            match self {
                Trie::Leaf(_) => Some(false),
                Trie::Node(_, boundary, _) => if *boundary {
                    Some(false)
                } else {
                    *boundary = true;
                    Some(true)
                },
            }
        } else {
            match self {
                Trie::Leaf(_) => {
                    *self = Trie::Node(*head, true, vec![Trie::from_word(tail.to_vec())]);
                    Some(true)
                }
                Trie::Node(_, _, children) => if let Some(position) = children
                    .iter_mut()
                    .position(|child| child.value() == &tail[0])
                {
                    children[position].insert(tail.to_vec())
                } else {
                    children.push(Trie::from_word(tail.to_vec()));
                    Some(true)
                },
            }
        }
    }
}

macro_rules! get_fn {
    (
        name: $name:ident,
        trie_type: $trie_type:ty,
        iter_fn: $iter:ident
    ) => {
        fn $name<'a>(trie: $trie_type, prefix: Vec<char>) -> Option<$trie_type> {
            if let Some((head, tail)) = prefix.split_first() {
                if head == trie.value() {
                    if tail.len() == 0 {
                        return Some(trie);
                    }

                    if let Trie::Node(_, _, children) = trie {
                        if let Some(child) =
                            children.$iter().find(|child| child.value() == &tail[0])
                        {
                            $name(child, tail.to_vec())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
}

get_fn!(name: trie_get, trie_type: &Trie, iter_fn: iter);
get_fn!(name: trie_get_mut, trie_type: &mut Trie, iter_fn: iter_mut);

#[cfg(test)]
mod tests {
    use super::*;
    use Trie::{Leaf, Node};

    #[test]
    fn trie_get() {
        let trie = Node(
            'a',
            false,
            vec![
                Leaf('b'),
                Node(
                    'c',
                    false,
                    vec![Leaf('d'), Node('e', false, vec![Leaf('f'), Leaf('g')])],
                ),
            ],
        );

        assert_eq!(
            *trie.get(vec!['a', 'c']).unwrap(),
            Node(
                'c',
                false,
                vec![Leaf('d'), Node('e', false, vec![Leaf('f'), Leaf('g')])],
            )
        )
    }

    #[test]
    fn trie_get_mut() {
        let mut trie = Node(
            'a',
            false,
            vec![
                Leaf('b'),
                Node(
                    'c',
                    false,
                    vec![Leaf('d'), Node('e', false, vec![Leaf('f'), Leaf('g')])],
                ),
            ],
        );

        *trie.get_mut(vec!['a', 'c', 'e']).unwrap().value_mut() = 'v';

        assert_eq!(
            trie,
            Node(
                'a',
                false,
                vec![
                    Leaf('b'),
                    Node(
                        'c',
                        false,
                        vec![Leaf('d'), Node('v', false, vec![Leaf('f'), Leaf('g')])],
                    ),
                ],
            )
        )
    }

    #[test]
    fn trie_insert() {
        let mut trie = Trie::from_word(vec!['a', 'c', 'e']);
        assert!(trie.insert(vec!['a', 'b']).unwrap());
        assert!(trie.insert(vec!['a', 'c', 'd']).unwrap());
        assert!(trie.insert(vec!['a', 'c', 'e', 'f']).unwrap());
        assert!(trie.insert(vec!['a', 'c', 'e', 'g']).unwrap());
        assert_eq!(
            trie,
            Node(
                'a',
                false,
                vec![
                    Node(
                        'c',
                        false,
                        vec![Node('e', true, vec![Leaf('f'), Leaf('g')]), Leaf('d')],
                    ),
                    Leaf('b'),
                ],
            )
        )
    }
}
