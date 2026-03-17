use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::bitset::BitSet;

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct Dawg {
    nodes: Vec<DawgNode>,
}

pub struct DawgNode {
    pub is_end_of_word: bool,
    pub children: BitSet,
    children_indices: Vec<usize>,
}

impl Dawg {
    pub fn read() -> Result<Dawg, Box<dyn Error>> {
        let mut dawg = Dawg::new();
        let file = File::open("words.txt")?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let word = line?;
            dawg.add(word.as_str());
        }

        Ok(dawg)
    }

    fn new() -> Dawg {
        let root = DawgNode::new();
        Dawg { nodes: vec![root] }
    }

    pub fn node(&self, index: usize) -> &DawgNode {
        &self.nodes[index]
    }

    pub fn add(&mut self, word: &str) {
        self.add_impl(word, 0);
    }

    fn add_impl(&mut self, suffix: &str, node_index: usize) {
        if suffix.is_empty() {
            self.nodes[node_index].is_end_of_word = true;
            return;
        }

        let letter: char = suffix.chars().next().unwrap();
        let letter_index = ALPHABET.binary_search(&letter).unwrap();

        let nodes_len = self.nodes.len();
        let node = self.node(node_index);

        if node.has_child(letter_index) {
            self.add_impl(&suffix[1..], node.child(letter_index));
        } else {
            let node = &mut self.nodes[node_index];
            let new_node = DawgNode::new();
            node.put_child(letter_index, nodes_len);
            self.nodes.push(new_node);
            self.add_impl(&suffix[1..], nodes_len);
        }
    }
}

impl DawgNode {
    fn new() -> DawgNode {
        DawgNode {
            is_end_of_word: false,
            children_indices: vec![0; ALPHABET.len()],
            children: BitSet::new(),
        }
    }

    pub fn child(&self, letter_index: usize) -> usize {
        self.children_indices[letter_index]
    }

    pub fn has_child(&self, letter_index: usize) -> bool {
        self.children_indices[letter_index] != 0
    }

    fn put_child(&mut self, letter_index: usize, child: usize) {
        self.children_indices[letter_index] = child;
        self.children.add(letter_index);
    }
}
