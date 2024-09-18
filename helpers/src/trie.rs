use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNode {
    end: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn is_end(&self) -> bool {
        self.end
    }

    pub fn get_child(&self, c: char) -> Option<&TrieNode> {
        self.children.get(&c)
    }
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }

        current_node.end = true;
    }

    pub fn get_root(&self) -> &TrieNode {
        &self.root
    }
}
