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
        if *head != self.key {
            return None;
        }

        if tail.len() == 0 {
            if self.boundary {
                Some(false)
            } else {
                self.boundary = true;
                Some(true)
            }
        } else {
            if let Some(position) = self
                .children
                .iter_mut()
                .position(|child| child.key == tail[0])
            {
                self.children[position].insert(tail.to_vec())
            } else {
                self.children.push(Trie::from_word(tail.to_vec()));
                Some(true)
            }
        }
    }

    pub fn remove(&mut self, word: Vec<char>) -> bool {
        trie_remove(self, true, word)
    }
}

macro_rules! get_fn {
    (
        name: $name:ident,
        trie_type: $trie_type:ty,
        iter_fn: $iter:ident
    ) => {
        fn $name<'a>(trie: $trie_type, prefix: Vec<char>) -> Option<$trie_type> {
            let (head, tail) = prefix
                .split_first()
                .expect("trie_get can't be called with empty prefix");

            if *head == trie.key {
                if tail.len() == 0 {
                    return Some(trie);
                }

                if let Some(child) = trie.children.$iter().find(|child| child.key == tail[0]) {
                    $name(child, tail.to_vec())
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

fn trie_remove(trie: &mut Trie, first: bool, word: Vec<char>) -> bool {
    let (last, rest) = word
        .split_last()
        .expect("trie_remove can't be called with empty word");

    if let Some(parent) = trie.get_mut(rest.to_vec()) {
        if let Some(pos) = parent.children.iter().position(|child| child.key == *last) {
            if parent.children[pos].children.len() != 0 {
                if first {
                    parent.children[pos].boundary = false;
                }

                return true;
            } else {
                parent.children.remove(pos);
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    trie_remove(trie, false, rest.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_get() {
        let trie = Trie {
            key: 'a',
            boundary: false,
            children: vec![
                Trie::empty('b'),
                Trie {
                    key: 'c',
                    boundary: false,
                    children: vec![
                        Trie::empty('d'),
                        Trie {
                            key: 'e',
                            boundary: false,
                            children: vec![Trie::empty('f'), Trie::empty('g')],
                        },
                    ],
                },
            ],
        };

        assert_eq!(
            *trie.get(vec!['a', 'c']).unwrap(),
            Trie {
                key: 'c',
                boundary: false,
                children: vec![
                    Trie::empty('d'),
                    Trie {
                        key: 'e',
                        boundary: false,
                        children: vec![Trie::empty('f'), Trie::empty('g')],
                    },
                ],
            }
        )
    }

    #[test]
    fn trie_get_mut() {
        let mut trie = Trie {
            key: 'a',
            boundary: false,
            children: vec![
                Trie::empty('b'),
                Trie {
                    key: 'c',
                    boundary: false,
                    children: vec![
                        Trie::empty('d'),
                        Trie {
                            key: 'e',
                            boundary: false,
                            children: vec![Trie::empty('f'), Trie::empty('g')],
                        },
                    ],
                },
            ],
        };

        trie.get_mut(vec!['a', 'c', 'e']).unwrap().key = 'v';

        assert_eq!(
            trie,
            Trie {
                key: 'a',
                boundary: false,
                children: vec![
                    Trie::empty('b'),
                    Trie {
                        key: 'c',
                        boundary: false,
                        children: vec![
                            Trie::empty('d'),
                            Trie {
                                key: 'v',
                                boundary: false,
                                children: vec![Trie::empty('f'), Trie::empty('g')],
                            },
                        ],
                    },
                ],
            }
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
            Trie {
                key: 'a',
                boundary: false,
                children: vec![
                    Trie {
                        key: 'c',
                        boundary: false,
                        children: vec![
                            Trie {
                                key: 'e',
                                boundary: true,
                                children: vec![Trie::empty('f'), Trie::empty('g')],
                            },
                            Trie::empty('d'),
                        ],
                    },
                    Trie::empty('b'),
                ],
            }
        )
    }

    #[test]
    fn trie_remove() {
        let mut trie = Trie {
            key: 'a',
            boundary: false,
            children: vec![
                Trie {
                    key: 'c',
                    boundary: false,
                    children: vec![
                        Trie {
                            key: 'e',
                            boundary: true,
                            children: vec![Trie::empty('f'), Trie::empty('g')],
                        },
                        Trie::empty('d'),
                    ],
                },
                Trie::empty('b'),
            ],
        };
        trie.remove(vec!['a', 'c', 'e', 'g']);

        assert_eq!(
            trie,
            Trie {
                key: 'a',
                boundary: false,
                children: vec![
                    Trie {
                        key: 'c',
                        boundary: false,
                        children: vec![
                            Trie {
                                key: 'e',
                                boundary: true,
                                children: vec![Trie::empty('f')],
                            },
                            Trie::empty('d'),
                        ],
                    },
                    Trie::empty('b'),
                ],
            }
        );

        trie.remove(vec!['a','c','e']);
        assert_eq!(
            trie,
            Trie {
                key: 'a',
                boundary: false,
                children: vec![
                    Trie {
                        key: 'c',
                        boundary: false,
                        children: vec![
                            Trie {
                                key: 'e',
                                boundary: false,
                                children: vec![Trie::empty('f')],
                            },
                            Trie::empty('d'),
                        ],
                    },
                    Trie::empty('b'),
                ],
            }
        );
    }
}
