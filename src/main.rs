extern crate trie;

use trie::Trie;

fn main() {
    let mut trie = Trie::from_word("hello😝world".chars().collect());
    trie.insert("hella".chars().collect());
    trie.insert("hello😝man!".chars().collect());

    println!("{:?}", trie);
}
