use std::slice;

use Trie;

pub struct Iter<'a> {
    root: &'a Trie,
    root_yielded: bool,
    iters: Vec<slice::Iter<'a, Trie>>,
    acc: Vec<char>,
}

impl Trie {
    pub fn iter(&self) -> Iter {
        Iter {
            root: self,
            root_yielded: false,
            iters: vec![],
            acc: vec![],
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Vec<char>> {
        if !self.root_yielded {
            self.iters.push(self.root.children.iter());
            if self.root.boundary {
                return Some(vec![self.root.key]);
            }
            self.root_yielded = true;
        }

        if self.acc.len() == 0 {
            self.acc.push(self.root.key);
        }

        loop {
            let next = match self.iters.last_mut() {
                Some(iter) => iter.next(),
                None => {
                    return None;
                }
            };

            match next {
                Some(child) => {
                    self.acc.push(child.key);
                    self.iters.push(child.children.iter());
                    if child.boundary {
                        return Some(self.acc.clone());
                    }
                }
                None => {
                    self.acc.pop();
                    self.iters.pop();
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_iter() {
        let mut trie = Trie::from_word("hello world!".chars().collect());
        trie.insert("hello man!".chars().collect());
        trie.insert("hey man!".chars().collect());
        trie.insert("hi🏞d".chars().collect());

        let a: Vec<Vec<char>> = trie.iter().collect();
        let b: Vec<Vec<char>> = vec![
            "hello world!".chars().collect(),
            "hello man!".chars().collect(),
            "hey man!".chars().collect(),
            "hi🏞d".chars().collect(),
        ];
        assert_eq!(a, b);
    }
}
