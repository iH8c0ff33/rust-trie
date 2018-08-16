extern crate trie;

use trie::Trie;

fn main() {
    let trie = Trie::new("hello😝world".chars().collect());

    println!("{:?}", trie);
}
