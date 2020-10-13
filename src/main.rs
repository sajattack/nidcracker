use openssl::sha::sha1;
//use sha1::{Sha1, Digest};
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
    let unk_nids: Vec<&str> = unk_nids_split.collect();


    wordlist.par_iter().for_each(|word| {
        let test_string = module_name.to_string() + word;
        //let mut hasher = Sha1::new();
        //hasher.update(test_string.as_str().as_bytes());
        //let hash = hasher.finalize();
        let hash = sha1(test_string.as_str().as_bytes());
        let test_nid = format!(
        "{:02X}{:02X}{:02X}{:02X}",
        hash[3], hash[2], hash[1], hash[0], 
        );
        if unk_nids.contains(&test_nid.as_str()) {
            println!("found match: {} = {}", test_nid, test_string);
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
        //let mut hasher = Sha1::new();
        //hasher.update(test_string.as_str().as_bytes());
        //let hash = hasher.finalize();
        let hash = sha1(test_string.as_str().as_bytes());

        let test_nid = format!(
        "{:02X}{:02X}{:02X}{:02X}",
        hash[3], hash[2], hash[1], hash[0], 
        );
        if unk_nids.contains(&test_nid.as_str()) {
            println!("found match: {} = {}", test_nid, test_string);
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
        //let mut hasher = Sha1::new();
        //hasher.update(test_string.as_str().as_bytes());
        //let hash = hasher.finalize();
        let hash = sha1(test_string.as_str().as_bytes());

        let test_nid = format!(
        "{:02X}{:02X}{:02X}{:02X}",
        hash[3], hash[2], hash[1], hash[0], 
        );
        if unk_nids.contains(&test_nid.as_str()) {
            println!("found match: {} = {}", test_nid, test_string);
        }
    });

    let mut wordlist_4perms = wordlist
        .iter()
        .permutations(4)
        .par_bridge()
        .into_par_iter(); 

    //for _ in 0..1_000_000_000 {
        //let perm = wordlist_4perms.next().unwrap();
        //let test_string = module_name.to_string() + perm[0] + perm[1] + perm[2] + perm[3]; 
        //println!("{}", test_string);
    //}

    wordlist_4perms.for_each(|perm| {
        let test_string = module_name.to_string() + perm[0] + perm[1] + perm[2] + perm[3];
        //let mut hasher = Sha1::new();
        //hasher.update(test_string.as_str().as_bytes());
        //let hash = hasher.finalize();
        let hash = sha1(test_string.as_str().as_bytes());
        let test_nid = format!(
        "{:02X}{:02X}{:02X}{:02X}",
        hash[3], hash[2], hash[1], hash[0], 
        );
        if unk_nids.contains(&test_nid.as_str()) {
            println!("found match: {} = {}", test_nid, test_string);
        }
    });

}
