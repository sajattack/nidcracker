use openssl::sha::sha1;
use std::fs::read_to_string;
use criterion::{criterion_group, criterion_main, Criterion};
use sha1::{Sha1, Digest};

fn other_crate(module_name: &str, wordlist: &Vec<&str>, unk_nids: &Vec<u32>) {
    let test_string = module_name.to_string() + wordlist[0] + wordlist[1] + wordlist[2];
    let mut hasher = Sha1::new();
    hasher.update(test_string.as_str().as_bytes());
    let hash = hasher.finalize();
    let hash4 = [hash[0], hash[1], hash[2], hash[3]];
    let test_nid = u32::from_le_bytes(hash4);
    if unk_nids.contains(&test_nid) {
        println!("found match: {:08X} = {}", test_nid, test_string);
    }
}

fn nidcracker_u32(module_name: &str, wordlist: &Vec<&str>, unk_nids: &Vec<u32>) {
    let test_string = module_name.to_string() + wordlist[0] + wordlist[1] + wordlist[2];
    let hash = sha1(test_string.as_str().as_bytes());
    let hash4 = [hash[0], hash[1], hash[2], hash[3]];
    let test_nid = u32::from_le_bytes(hash4);
    if unk_nids.contains(&test_nid) {
        println!("found match: {:08X} = {}", test_nid, test_string);
    }
}

fn nidcracker_string(module_name: &str, wordlist: &Vec<&str>, unk_nids: &Vec<&str>) {
    let test_string = module_name.to_string() + wordlist[0] + wordlist[1] + wordlist[2];
    let hash = sha1(test_string.as_str().as_bytes());
    let test_nid = format!("{:02X}{:02X}{:02X}{:02X}", hash[3], hash[2], hash[1], hash[0]);
    if unk_nids.contains(&test_nid.as_str()) {
        println!("found match: {} = {}", test_nid, test_string);
    }
}

fn bench(c: &mut Criterion) {
    let module_name = "sceSysreg";

    let wordlist_string = read_to_string("wordlist.txt").unwrap();
    let wordlist_split = wordlist_string.split('\n');
    let wordlist: Vec<&str> = wordlist_split.collect();

    let unknown_nids_string = read_to_string("unknown_nids.txt").unwrap();
    let unk_nids_split = unknown_nids_string.split('\n');
    let unk_nids_split_string = unk_nids_split.clone();
    let unk_nids_u32: Vec<u32> = unk_nids_split.map(|s| u32::from_str_radix(s, 16).unwrap()).collect();
    let unk_nids_string: Vec<&str> = unk_nids_split_string.collect();

    c.bench_function("nidcracker_u32", |b| b.iter(|| { nidcracker_u32(module_name, &wordlist, &unk_nids_u32)}));
    c.bench_function("nidcracker_string", |b| b.iter(|| { nidcracker_string("sceSysreg", &wordlist, &unk_nids_string)}));
    c.bench_function("other_sha1_crate", |b| b.iter(|| { other_crate(module_name, &wordlist, &unk_nids_u32)}));

}

criterion_group!(benches, bench);
criterion_main!(benches);
