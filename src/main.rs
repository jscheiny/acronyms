use std::io::{self, BufRead};

use crate::acronyms::{Acronyms, NameCoordinate};

mod acronyms;
mod bitset;
mod dawg;

fn main() {
    let mut names: Vec<Vec<String>> = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let aliases = line
            .expect("Could not read line")
            .split(',')
            .map(|elem| elem.trim().to_owned())
            .collect::<Vec<String>>();
        names.push(aliases);
    }

    let mut longest: Option<(String, Vec<NameCoordinate>)> = None;
    Acronyms::find(&names, |acronym, coordinates| {
        if let Some(longest_instance) = longest.as_ref() {
            if acronym.len() > longest_instance.0.len() {
                longest = Some((acronym.clone(), coordinates.clone()))
            }
        } else {
            longest = Some((acronym.clone(), coordinates.clone()))
        }

        print_acronym(&names, acronym, coordinates);
    });

    if let Some(longest) = longest {
        println!("Longest acronym ({}):", longest.0.len());
        print_acronym(&names, &longest.0, &longest.1);
    }
}

fn print_acronym(names: &[Vec<String>], acronym: &String, coordinates: &[NameCoordinate]) {
    print!("{}: ", acronym);
    for (index, coordinate) in coordinates.iter().enumerate() {
        let NameCoordinate {
            name_index,
            alias_index,
        } = *coordinate;
        let primary_name = &names[name_index][0];
        if alias_index == 0 {
            print!("{}", primary_name);
        } else {
            print!("{} ({})", primary_name, names[name_index][alias_index])
        }
        if index < coordinates.len() - 2 {
            print!(", ");
        }
        if index == coordinates.len() - 2 {
            print!(", and ");
        }
    }
    println!();
}
