pub mod trie_node;
pub mod trie_traversal;

#[derive(Debug,PartialEq)]
pub struct Trie {
    pub key: char,
    pub boundary: bool,
    children: Vec<Trie>,
}
