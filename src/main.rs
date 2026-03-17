use std::io::{self, BufRead};

use crate::acronyms::Acronyms;

mod acronyms;
mod bitset;
mod dawg;

fn main() {
    let mut names = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let name = line.expect("Could not read line");
        names.push(name);
    }

    Acronyms::find(&names, &|acronym, indices| {
        print!("{}: ", acronym);
        for (index, name_index) in indices.iter().enumerate() {
            print!("{}", names[*name_index]);
            if index < indices.len() - 2 {
                print!(", ");
            }
            if index == indices.len() - 2 {
                print!(", and ");
            }
        }
        println!();
    });
}
