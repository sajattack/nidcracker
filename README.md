# nidcracker
Bruteforces PSP NIDs in parallel with Rust.

## Install Rust
https://rustup.rs/

## Modify the input files
- `module-names.txt` is the list of prefixes to try in combination with the words in wordlist.txt
- `wordlist.txt` is a list of words to try permutations of to generate a NID
- `unknown_nids.txt` is a list of nids to match against

## Let 'er rip
`cargo run --release`
