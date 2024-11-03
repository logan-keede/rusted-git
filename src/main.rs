#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use sha1::{Digest, Sha1};

extern crate flate2;
use flate2::read::ZlibDecoder;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use hex;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;
use std::ptr::null;
// use std::ptr::hash;
// use flate2::write::ZlibEncoder::<W>::new;
// use std::io::BufReader;

fn compute_sha1_hash(file_path: &str) -> io::Result<String> {
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(file_path)?;
    let _n = io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    // Format the hash as a hexadecimal string
    Ok(hash.iter().map(|byte| format!("{:02x}", byte)).collect())
}
fn compute_sha1_hash_str(strin: &str) {
    let mut hasher = Sha1::new();
    hasher.update(strin);
    let hash = hasher.finalize();
    println!("{:x}", hash);
}

fn read_tree(mut null_indices: Vec<usize>, decompressed_data: Vec<u8>) {
    let mut temp = &decompressed_data[null_indices[0] + 1..null_indices[1]];
    print!("{}  ", String::from_utf8_lossy(temp));
    // println!("{}", decompressed_data.len());
    for i in 1..null_indices.len() {
        temp = &decompressed_data[null_indices[i] + 1..null_indices[i] + 21];
        // println!("{}", String::from_utf8_lossy(temp));
        let hex_string = hex::encode(&temp);
        println!("{}", hex_string);
        if i != null_indices.len() - 1 {
            temp = &decompressed_data[null_indices[i] + 21..null_indices[i + 1]];
            print!("{}  ", String::from_utf8_lossy(temp));
        } else {
            temp = &decompressed_data[null_indices[i] + 21..];
            print!("{}  ", String::from_utf8_lossy(temp));
        }
    }
}

fn read_cat_file(sha: &String) {
    // println!("{}", &sha);
    let sha_file_path = PathBuf::from(format!(".git/objects/{}/{}", &sha[0..2], &sha[2..]));
    let sha_file = fs::File::open(&sha_file_path);

    let mut decoder = ZlibDecoder::new(sha_file.unwrap());
    let mut decompressed_data = Vec::new();

    let _ = decoder.read_to_end(&mut decompressed_data);

    let bin = String::from_utf8_lossy(&decompressed_data);
    let parts = bin.split("\0").collect::<Vec<&str>>();
    let null_indices: Vec<usize> = decompressed_data
        .iter() // Create an iterator over the byte vector
        .enumerate() // Get index and value as a tuple
        .filter_map(|(index, &byte)| {
            // Filter out null characters
            if byte == 0 {
                // Check if the byte is a null character
                Some(index) // If yes, return the index
            } else {
                None // If not, return None
            }
        })
        .collect();

    let temp = &parts[0][0..4];
    if temp == String::from("tree") {
        println!("{:?}", null_indices);
        println!("{:?}", parts);
        // let an_arr = &decompressed_data[null_indices[null_indices.len() - 1]..];
        read_tree(null_indices, decompressed_data);
    } else if temp == String::from("comm") {
        let joined = &decompressed_data[null_indices[0] + 1..];
        println!("{}", String::from_utf8_lossy(joined));
    } else if temp == String::from("blob") {
        let joined = &decompressed_data[null_indices[0] + 1..];

        println!("{}", String::from_utf8_lossy(joined));
    }
}

fn hash_object(file: &String) {
    let file_path = PathBuf::from(file);
    let sha = compute_sha1_hash(file).unwrap();
    print!("{}", sha);
    let sha_file_path = PathBuf::from(format!(".git/objects/{}/{}", &sha[0..2], &sha[2..]));
    if let Some(parent) = sha_file_path.parent() {
        let _ = fs::create_dir_all(parent); // Creates directories if they don't exist
    }

    let sha_file = File::create(&sha_file_path).unwrap();
    let mut encoder = ZlibEncoder::new(sha_file, Compression::default());
    let to_write = fs::read_to_string(file_path).unwrap();
    let content = format!("blob {}\0{}", to_write.len(), to_write);
    let _ = encoder.write_all(content.as_bytes());
    let _ = encoder.finish();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "cat-file" {
        if let Some(pos) = args.iter().position(|arg| arg == "-p") {
            // Get the argument following "-p", if it exists
            if let Some(file) = args.get(pos + 1) {
                read_cat_file(file);
            } else {
                eprintln!("Error: No value provided for -p flag");
            }
        // let mut decoder= ZlibDecoder::new();
        } else {
            println!("-p flag not found");
        }
    } else if args[1] == "hash-object" {
        if let Some(pos) = args.iter().position(|arg| arg == "-w") {
            // Get the argument following "-p", if it exists
            if let Some(file) = args.get(pos + 1) {
                hash_object(file);
            } else {
                eprintln!("Error: No value provided for -p flag");
            }
        // let mut decoder= ZlibDecoder::new();
        } else {
            println!("-w flag not found");
        }
    } else if args[1] == "hash" {
        compute_sha1_hash_str(&args[2]);
    } else {
        println!("unknown command: {}", args[1])
    }
}
