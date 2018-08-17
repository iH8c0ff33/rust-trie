extern crate trie;

use trie::Trie;

fn main() {
    let mut trie = Trie::from_word("hello😝world".chars().collect());
    trie.insert("hella".chars().collect());
    trie.insert("hello😝man!".chars().collect());
    trie.insert("hello".chars().collect());

    println!("{:?}", trie.get("hell".chars().collect()).unwrap());

    println!(
        "{:?}",
        trie.iter()
            .map(|vec| vec.iter().collect::<String>())
            .collect::<Vec<String>>()
    );
}
