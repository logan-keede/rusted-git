// use std::{fs, io};
// use sha1::Sha1;
// use core::hash;

#![allow(warnings)]

// mod compute_hash;

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::{self, Read, Write};

use std::path::PathBuf;

use sha1::{Digest, Sha1};

extern crate flate2;
use flate2::read::ZlibDecoder;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use hex;

use std::path::Path;

pub fn compute_sha1_hash(file_path: &str) -> Vec<u8> {
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(file_path).unwrap();
    let _n = io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize().to_vec();
    return hash;
}

pub fn compute_hash_hex(hash: &Vec<u8>) -> String {
    return hash
        .iter()
        .map(|byte: &u8| format!("{:02x}", byte))
        .collect();
}

pub fn compute_sha1_hash_str(strin: &str) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(strin);
    let hash = hasher.finalize();
    println!("{:x}", hash);
    return hash.to_vec();
}

pub fn compute_sha1_bytes(bytes: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let hash = hasher.finalize().to_vec();
    return hash;
}
