use openssl::sha::sha1;
use rayon::prelude::*;
use rayon::iter::ParallelBridge;
use std::fs::read_to_string;
use itertools::Itertools;

fn main() {
    //let module_names_string = read_to_string("module-names.txt").unwrap();
    //let module_names_split = module_names_string.split('\n');
    //let module_names: Vec<&str> = module_names_split.collect();

    let module_name = "sceSysreg";

    let wordlist_string = read_to_string("wordlist.txt").unwrap();
    let wordlist_split = wordlist_string.split('\n');
    let wordlist: Vec<&str> = wordlist_split.collect();

    let unknown_nids_string = read_to_string("unknown_nids.txt").unwrap();
    let unk_nids_split = unknown_nids_string.split('\n');
    let unk_nids: Vec<u32> = unk_nids_split.map(|s| u32::from_str_radix(s, 16).unwrap()).collect();

    wordlist.par_iter().for_each(|word| {
        let test_string = module_name.to_string() + word;
        let hash = sha1(test_string.as_str().as_bytes());
        let hash4 = [hash[0], hash[1], hash[2], hash[3]];
        let test_nid = u32::from_le_bytes(hash4);
        if unk_nids.contains(&test_nid) {
            println!("found match: {:08X} = {}", test_nid, test_string);
        }
    });
 
    let wordlist_2perms = wordlist
        .iter()
        .permutations(2)
        .into_iter()
        .par_bridge()
        .into_par_iter(); 

    wordlist_2perms.for_each(|perm| {
        let test_string = module_name.to_string() + perm[0] + perm[1];
        let hash = sha1(test_string.as_str().as_bytes());
        let hash4 = [hash[0], hash[1], hash[2], hash[3]];
        let test_nid = u32::from_le_bytes(hash4);
        if unk_nids.contains(&test_nid) {
            println!("found match: {:08X} = {}", test_nid, test_string);
        }
    });
 
    let wordlist_3perms = wordlist
        .iter()
        .permutations(3)
        .into_iter()
        .par_bridge()
        .into_par_iter(); 

    wordlist_3perms.for_each(|perm| {
        let test_string = module_name.to_string() + perm[0] + perm[1] + perm[2];
        let hash = sha1(test_string.as_str().as_bytes());
        let hash4 = [hash[0], hash[1], hash[2], hash[3]];
        let test_nid = u32::from_le_bytes(hash4);
        if unk_nids.contains(&test_nid) {
            println!("found match: {:08X} = {}", test_nid, test_string);
        }
    });

    let wordlist_4perms = wordlist
        .iter()
        .permutations(4)
        .par_bridge()
        .into_par_iter(); 

    wordlist_4perms.for_each(|perm| {
        let test_string = module_name.to_string() + perm[0] + perm[1] + perm[2] + perm[3];
        let hash = sha1(test_string.as_str().as_bytes());
        let hash4 = [hash[0], hash[1], hash[2], hash[3]];
        let test_nid = u32::from_le_bytes(hash4);
        if unk_nids.contains(&test_nid) {
            println!("found match: {:08X} = {}", test_nid, test_string);
        }
    });
}
