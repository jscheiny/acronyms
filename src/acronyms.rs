use std::collections::{HashMap, HashSet};

use crate::dawg::{ALPHABET, Dawg};

pub struct Acronyms {
    dawg: Dawg,
    name_map: HashMap<char, Vec<usize>>,
}

impl Acronyms {
    pub fn find(names: &[String], on_find: &impl Fn(&String, &Vec<usize>)) {
        Self::new(names).find_acronyms(
            0,
            &mut String::new(),
            &mut HashSet::new(),
            &mut vec![],
            on_find,
        );
    }

    fn new(names: &[String]) -> Self {
        let dawg = Dawg::read().unwrap();
        Self {
            dawg,
            name_map: make_first_letter_lookup(names),
        }
    }

    fn find_acronyms(
        &self,
        node_index: usize,
        word: &mut String,
        used_names_set: &mut HashSet<usize>,
        used_names_vec: &mut Vec<usize>,
        on_find: &impl Fn(&String, &Vec<usize>),
    ) {
        let node = self.dawg.node(node_index);
        if node.is_end_of_word {
            on_find(word, used_names_vec);
        }
        node.children.for_each(|letter_index| {
            let letter = ALPHABET[letter_index];
            let name_indices = self.name_map.get(&letter);

            if let Some(name_indices) = name_indices {
                for name_index in name_indices {
                    if !used_names_set.contains(name_index) {
                        let name_index = *name_index;
                        used_names_set.insert(name_index);
                        used_names_vec.push(name_index);
                        word.push(letter);

                        self.find_acronyms(
                            node.child(letter_index),
                            word,
                            used_names_set,
                            used_names_vec,
                            on_find,
                        );

                        used_names_set.remove(&name_index);
                        used_names_vec.pop();
                        word.pop();
                        break;
                    }
                }
            }
        });
    }
}

fn make_first_letter_lookup(names: &[String]) -> HashMap<char, Vec<usize>> {
    let mut table: HashMap<char, Vec<usize>> = HashMap::new();
    for (index, name) in names.iter().enumerate() {
        let first_letter = name.chars().next().unwrap();
        table.entry(first_letter).or_default().push(index);
    }

    table
}
