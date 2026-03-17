use std::collections::HashMap;

use crate::{
    bitset::BitSet,
    dawg::{ALPHABET, Dawg},
};

pub struct Acronyms {
    dawg: Dawg,
    name_map: HashMap<char, Vec<NameCoordinate>>,
}

#[derive(Clone, Copy, Debug)]
pub struct NameCoordinate {
    pub name_index: usize,
    pub alias_index: usize,
}

impl Acronyms {
    pub fn find(names: &[Vec<String>], mut on_find: impl FnMut(&String, &Vec<NameCoordinate>)) {
        Self::new(names).find_acronyms(
            0,
            &mut String::new(),
            &mut BitSet::new(),
            &mut vec![],
            &mut on_find,
        );
    }

    fn new(names: &[Vec<String>]) -> Self {
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
        used_names_set: &mut BitSet,
        used_names_vec: &mut Vec<NameCoordinate>,
        on_find: &mut impl FnMut(&String, &Vec<NameCoordinate>),
    ) {
        let node = self.dawg.node(node_index);
        if node.is_end_of_word {
            on_find(word, used_names_vec);
        }
        node.children.for_each(|letter_index| {
            let letter = ALPHABET[letter_index];
            let Some(name_coordinates) = self.name_map.get(&letter) else {
                return;
            };

            for coordinate in name_coordinates {
                let name_index = coordinate.name_index;
                if !used_names_set.contains(name_index) {
                    used_names_set.add(name_index);
                    used_names_vec.push(*coordinate);
                    word.push(letter);

                    self.find_acronyms(
                        node.child(letter_index),
                        word,
                        used_names_set,
                        used_names_vec,
                        on_find,
                    );

                    used_names_set.remove(name_index);
                    used_names_vec.pop();
                    word.pop();
                    break;
                }
            }
        });
    }
}

fn make_first_letter_lookup(names: &[Vec<String>]) -> HashMap<char, Vec<NameCoordinate>> {
    let mut table: HashMap<char, Vec<NameCoordinate>> = HashMap::new();
    for (name_index, names) in names.iter().enumerate() {
        for (alias_index, alias) in names.iter().enumerate() {
            let first_letter = alias.chars().next().unwrap().to_ascii_uppercase();
            table.entry(first_letter).or_default().push(NameCoordinate {
                name_index,
                alias_index,
            });
        }
    }

    table
}
