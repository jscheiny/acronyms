# Acronym finder
Finds an acronym that's a valid English word (as determined by `words.txt`) using a subset of the names provided via standard in. For example. Suppose you have the names, Bob, Ingrid, Phoebe, Zoey. Then this would produce the following result:

```
BI: Bob, and Ingrid
BIZ: Bob, Ingrid, and Zoey
PI: Phoebe, and Ingrid
ZIP: Zoey, Ingrid, and Phoebe
```

## Usage
Make sure you have [rust installed](https://rust-lang.org/tools/install/).

Provide your names in a text file (e.g. `names.txt`). Each line should be it's own name and the first letter of each name should be an uppercase ASCII English letter.

Run the following command:

```
cargo run < names.txt
```