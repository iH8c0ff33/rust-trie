pub mod trie_node;
pub mod trie_traversal;

#[derive(Debug)]
pub enum Trie {
    Leaf(char),
    Node(char, bool, Vec<Trie>),
}
