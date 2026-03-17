# Acronym finder
Finds an acronym that's a valid English word (as determined by `words.txt`) using a subset of the names provided via standard in. For example. Suppose you have the names, Bob, Ingrid, Phoebe, Zoey. Then this would produce the following result:

```
BI: Bob, and Ingrid
BIZ: Bob, Ingrid, and Zoey
PI: Phoebe, and Ingrid
ZIP: Zoey, Ingrid, and Phoebe
```

If there are multiple names provided that start with the same letter, then the first one provided will be used first, and then the second one second and so on and so forth. Multiple acronyms are not provided for each possible name with the same starting letter. For example. If you have Alice, Alex, and Bob, you will only get `AB: Alice, Bob` and not `AB: Alex, Bob`

Aliases can be provided by giving a comma separated list of aliases for a person in a single line. For example you might want to provide nicknames like:

```
Elizabeth,Lizzy,Beth
```

That person will only be used once in an acronym but can now serve as either Robert or Bob depending on the letter needed.

## Usage
Make sure you have [rust installed](https://rust-lang.org/tools/install/).

Provide your names in a text file (e.g. `names.txt`). Each line should be it's own name and the first letter of each name should be an uppercase ASCII English letter.

Run the following command:

```
cargo run < names.txt
```