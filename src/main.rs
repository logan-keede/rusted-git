#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::path::PathBuf;
use std::io::{self, Read};

use sha1::{Sha1, Digest};

extern crate flate2;
use flate2::read::ZlibDecoder;


use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::fs::File;
// use flate2::write::ZlibEncoder::<W>::new;
// use std::io::BufReader;


fn compute_sha1_hash(file_path: &str) -> io::Result<String> {
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(file_path)?;
    let n = io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    // Format the hash as a hexadecimal string
    Ok(hash.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect())
}


fn read_cat_file(sha: &String){

    let sha_file_path = PathBuf::from(format!(".git/objects/{}/{}", &sha[0..2], &sha[2..]));
    let sha_file = fs::File::open(&sha_file_path);
    let mut decoder= ZlibDecoder::new(sha_file.unwrap());
    let mut decompressed_data = String::new();
    let _ = decoder.read_to_string(&mut decompressed_data);
    print!("{}", &decompressed_data[8..]);
}

fn hash_object(file: &String){
    let file_path = PathBuf::from(file);
    let sha = compute_sha1_hash(file).unwrap();

    let sha_file_path = PathBuf::from(format!(".git/objects/{}/{}", &sha[0..2], &sha[2..]));
    if let Some(parent) = sha_file_path.parent() {
        fs::create_dir_all(parent);  // Creates directories if they don't exist
    }

    let sha_file = File::create(&sha_file_path).unwrap();
    let mut encoder= ZlibEncoder::new(sha_file, Compression::default());
    let to_write = fs::read_to_string(file_path).unwrap();
    let content = format!("blob {}\0{}", to_write.len(), to_write);
    encoder.write_all(content.as_bytes());
    encoder.finish();
    // let compressed_data = encoder.finish().unwrap();

    // sha_file.unwrap().write_all(compressed_data.as_slice()).unwrap();
    // println!("{}", &compressed_data.as_slice()[8..]);
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
    }
    else if args[1] == "cat-file" {
        if let Some(pos) = args.iter().position(|arg| arg == "-p") {
            // Get the argument following "-p", if it exists
            if let Some(file) = args.get(pos + 1) {
                read_cat_file(file);
            } else {
                eprintln!("Error: No value provided for -p flag");
            }
        // let mut decoder= ZlibDecoder::new();
        }   else { println!("-p flag not found"); }
    }
    else if args[1] == "hash-object" {
        if let Some(pos) = args.iter().position(|arg| arg == "-w") {
            // Get the argument following "-p", if it exists
            if let Some(file) = args.get(pos + 1) {
                hash_object(file);
            } else {
                eprintln!("Error: No value provided for -p flag");
            }
        // let mut decoder= ZlibDecoder::new();
        }   else { println!("-w flag not found"); }
    }
    else {
        println!("unknown command: {}", args[1])
    }



}
