// use core::hash;

#![allow(warnings)]

mod compute_hash;
use compute_hash::*;

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

fn determine_type(sha: &String) -> String {
    let sha_file_path = PathBuf::from(format!(".git/objects/{}/{}", &sha[0..2], &sha[2..]));
    println!("determine type for {:?}", sha);
    let sha_file = fs::File::open(&sha_file_path);

    let mut decoder = ZlibDecoder::new(sha_file.unwrap());
    let mut decompressed_data = Vec::new();
    let _ = decoder.read_to_end(&mut decompressed_data);
    let st = String::from_utf8_lossy(&decompressed_data[0..4]).to_string();
    return st;
}

fn read_tree(null_indices: Vec<usize>, decompressed_data: Vec<u8>) {
    let mut mods = Vec::new();
    let mut names = Vec::new();
    let mut hashes = Vec::new();
    // let mut object_types = Vec::new();
    let mut temp = &decompressed_data[null_indices[0] + 1..null_indices[1]];
    // let mut splite
    // let mut start = 0;
    for (index, &byte) in temp.iter().enumerate() {
        if byte == 32 {
            if index >= 6 {
                mods.push(format!("{}", String::from_utf8_lossy(&temp[0..index])));
            } else {
                mods.push(format!("0{}", String::from_utf8_lossy(&temp[0..index])));
            }

            names.push(&temp[index + 1..]);
        }
    }

    // println!("{}", decompressed_data.len());
    for i in 1..null_indices.len() {
        temp = &decompressed_data[null_indices[i] + 1..null_indices[i] + 21];
        // println!("{}", String::from_utf8_lossy(temp));
        let hex_string = hex::encode(&temp);
        hashes.push(hex_string);

        if i != null_indices.len() - 1 {
            temp = &decompressed_data[null_indices[i] + 21..null_indices[i + 1]];
            // println!("{}", String::from_utf8_lossy(temp));
            // println!("{:?}", temp);
            for (index, &byte) in temp.iter().enumerate() {
                if byte == 32 {
                    if index >= 6 {
                        mods.push(format!("{}", String::from_utf8_lossy(&temp[0..index])));
                    } else {
                        mods.push(format!("0{}", String::from_utf8_lossy(&temp[0..index])));
                    }

                    names.push(&temp[index + 1..]);
                }
            }
        }
    }
    // print!("{}  ", mods.len());
    for i in 0..mods.len() {
        println!(
            "{}    {}   {}    ",
            mods[i],
            // object_types[i],
            hashes[i],
            String::from_utf8_lossy(names[i])
        );
    }
}

fn read_cat_file(sha: &String) {
    // println!("{}", &sha);
    let sha_file_path = PathBuf::from(format!(".git/objects/{}/{}", &sha[0..2], &sha[2..]));
    let sha_file = fs::File::open(&sha_file_path);

    let mut decoder = ZlibDecoder::new(sha_file.unwrap());
    let mut decompressed_data = Vec::new();

    let _ = decoder.read_to_end(&mut decompressed_data);


    let mut null_indices: Vec<usize> = Vec::new(); // Initialize an empty vector to store indices
    let mut index = 0;
    while index < decompressed_data.len() {
        if decompressed_data[index] == 0 {
            // Check if the byte is a null character
            null_indices.push(index); // If yes, push the index to the vector
            index += 20;
        } else {
            index += 1;
        }
        // Increment the index
    }

    let temp = String::from_utf8_lossy(&mut decompressed_data[0..4]).to_string();
    if temp == String::from("tree") {
        read_tree(null_indices, decompressed_data);
    } else if temp == String::from("comm") {
        let joined = &decompressed_data[null_indices[0] + 1..];
        println!("{}", String::from_utf8_lossy(joined));
    } else if temp == String::from("blob") {
        let joined = &decompressed_data[0 ..];

        println!("{} {}", String::from_utf8(joined.to_vec()).unwrap(), joined.len());
    }
}

fn hash_object(file: &String) -> Vec<u8> {
    let file_path = PathBuf::from(file);

    let to_write = fs::read_to_string(file_path).unwrap();

    let normalized_content = to_write.replace("\r\n", "\n");

    let content = format!("blob {}\0{}", normalized_content.len(), normalized_content);
    let mut hello: Vec<u8> = Vec::new();
    let mut encoder = ZlibEncoder::new(&mut hello, Compression::default());

    let _ = encoder.write_all(content.as_bytes());
    let _ = encoder.finish();

    let sha_pure = compute_sha1_bytes(&content.into_bytes());
    let sha = compute_hash_hex(&sha_pure);
    println!("{}", sha);

    let sha_file_path = PathBuf::from(format!(".git/objects/{}/{}", &sha[0..2], &sha[2..]));

    if let Some(parent_dir) = sha_file_path.parent() {
        fs::create_dir_all(parent_dir).unwrap();
    }
    fs::write(&sha_file_path, &hello);
    sha_pure
}

fn write_tree(dir: &Path) -> Vec<u8> {
    let mut mods = Vec::new();
    let mut names = Vec::new();
    let mut hashes = Vec::new();

    if dir.is_dir() {

        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();

            let path = entry.path();
            if path.is_dir() {
                let file_name = format!("{}", path.to_string_lossy().to_string());
                if path != PathBuf::from("./.git")
                    && path != PathBuf::from("./target")
                    && path != PathBuf::from("./test")
                {
                    // let file_nam
                    let _ = write_tree(&path);
                }
            } else {
                let file_name = format!("{}", entry.file_name().to_str().unwrap());

                if let Some(extension) = path.extension() {
                    if extension != "TAG" {
                        let path = path.to_string_lossy().to_string();
                        let sha_pure = hash_object(&path);
                        hashes.push(sha_pure);
                        names.push(file_name.as_bytes().to_vec());
                        mods.push("100644".as_bytes().to_vec());
                    }
                }
            }
        }
    }

    let mut header = Vec::new();
    let mut write = Vec::new();

    for ind in 0..mods.len() {
        write.append(&mut mods[ind]);
        write.push(32);
        write.append(&mut names[ind]);
        write.push(0);
        write.append(&mut hashes[ind]);
    }
    // let mut head = format!("tree {}\0", write.len()).as_bytes();
    header.append(&mut format!("tree {}\0", write.len()).as_bytes().to_vec());
    header.append(&mut write);
    println!("{:?}", String::from_utf8_lossy(&header));

    let mut hello: Vec<u8> = Vec::new();
    let mut encoder = ZlibEncoder::new(&mut hello, Compression::default());

    // let content = content.as_bytes();
    let _ = encoder.write_all(&header);
    let _ = encoder.finish();
    let sha_pure = compute_sha1_bytes(&header);
    let sha = compute_hash_hex(&sha_pure);
    println!("{}", sha);
    let sha_file_path = PathBuf::from(format!(".git/objects/{}/{}", &sha[0..2], &sha[2..]));
    println!("{:?}", sha_file_path);

    let _ = fs::write(&sha_file_path, &hello);

    return sha_pure;
}

fn main() {
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
                let _ = hash_object(file);
            } else {
                eprintln!("Error: No value provided for -p flag");
            }
        // let mut decoder= ZlibDecoder::new();
        } else {
            println!("-w flag not found");
        }
    } else if args[1] == "hash" {
        compute_sha1_hash_str(&args[2]);
    } else if args[1] == "write-tree" {
        let dir = Path::new(".");
        let _ = write_tree(dir);
    } else {
        println!("unknown command: {}", args[1])
    }
}
