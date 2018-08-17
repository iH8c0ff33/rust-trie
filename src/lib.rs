pub mod trie_node;
pub mod trie_traversal;
pub mod trie_iter;

#[derive(Debug,PartialEq)]
pub struct Trie {
    pub key: char,
    pub boundary: bool,
    children: Vec<Trie>,
}
