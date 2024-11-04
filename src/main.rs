// use core::hash;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::{self, Read, Write};
// use std::os::linux::net::SocketAddrExt;
use std::path::PathBuf;

use sha1::{Digest, Sha1};

extern crate flate2;
use flate2::read::ZlibDecoder;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use hex;
use std::fs::File;
// use std::io::prelude::*;

use std::path::Path;

// use std::ptr::hash;
// use flate2::write::ZlibEncoder::<W>::new;
// use std::io::BufReader;

fn compute_sha1_hash(file_path: &str) -> Vec<u8> {
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(file_path).unwrap();
    let _n = io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize().to_vec();
    // for i in 0..hash.len() {
    //     print!("{}", hash[i]);
        
    // }
    
    // println!("{:?}", hash);
    // println!("{}", compute_hash_hex(&hash));
    return hash;
    // Format the hash as a hexadecimal string
    // return hash.iter().map(|byte: &u8| format!("{:02x}", byte)).collect();
}

fn compute_hash_hex(hash: &Vec<u8>) -> String {
    return hash.iter().map(|byte: &u8| format!("{:02x}", byte)).collect();
}

fn compute_sha1_hash_str(strin: &str) {
    let mut hasher = Sha1::new();
    hasher.update(strin);
    let hash = hasher.finalize();
    println!("{:x}", hash); 
}

fn determine_type(sha: &String) -> String {
    let sha_file_path = PathBuf::from(format!(".git/objects/{}/{}", &sha[0..2], &sha[2..]));
    let sha_file = fs::File::open(&sha_file_path);

    let mut decoder = ZlibDecoder::new(sha_file.unwrap());
    let mut decompressed_data = Vec::new();

    let _ = decoder.read_to_end(&mut decompressed_data);
    let st =  String::from_utf8_lossy(&decompressed_data[0..4]).to_string();
    return st;
}

fn read_tree(null_indices: Vec<usize>, decompressed_data: Vec<u8>) {
    let mut mods = Vec::new();
    let mut names = Vec::new();
    let mut hashes = Vec::new();
    let mut object_types = Vec::new();
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
        object_types.push(determine_type(&hex_string));
        hashes.push(hex_string);
        
        // println!("{}", hex_string);

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
            "{}    {}   {}    {}",
            mods[i],
            object_types[i],
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

    let bin = String::from_utf8_lossy(&decompressed_data);
    let parts = bin.split("\0").collect::<Vec<&str>>();
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

    let temp = &parts[0][0..4];
    if temp == String::from("tree") {
        // println!("{:?}", null_indices);
        // println!("{:?}", parts);
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

fn hash_object(file: &String) -> Vec<u8> {
    let file_path = PathBuf::from(file);
    let sha_pure = compute_sha1_hash(file);
    let sha = compute_hash_hex(&sha_pure);
    println!("{}", sha);
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
    return sha_pure;
}


fn write_tree(dir: &Path) -> io::Result<()> {
    let mut mods = Vec::new();
    let mut names = Vec::new();
    let mut hashes = Vec::new();
    // print!("{:?}", fs::read_dir(dir)?);

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            println!("{:?}", entry?)
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            
            let path = entry.path();
            if path.is_dir() {
                let file_name = format!("{}", path.to_string_lossy().to_string());
                // println!("{:?}", path.to_string_lossy().to_string());
                // let path = path.to_string_lossy().to_string();
                if file_name != "./.git" && file_name!="./target" && file_name!="./test" {
                    // let file_nam
                    let _ = write_tree(&path);
                    // println!("hawa mehal");    
                    // names.push(file_name.as_bytes().to_vec());
                    // mods.push("100065".as_bytes().to_vec());
                }
                
                // // let sha_pure = hash_object(&file_name);
                // // hashes.push(sha_pure);
                
                

            } else {
                let file_name = format!("{}", entry.file_name().to_str().unwrap());
                // let path = path.to_str().unwrap();
                // println!("{:?}", path.to_str().unwrap());
                if let Some(extension) = path.extension(){
                    if extension != "TAG"{
                        let path = path.to_string_lossy().to_string();
                    let sha_pure = hash_object(&path);
                    hashes.push(sha_pure);
                    names.push(file_name.as_bytes().to_vec());
                    mods.push("40000".as_bytes().to_vec());
                }
    
                }
                // if entry.file_type() ==  {
                
                // }
                // let sha_pure = hash_object(&file_name);
                // hashes.push(sha_pure);
                // names.push(file_name);
                // mods.push("40000");

            }
        }
    }

    
    let mut write = Vec::new();
    
    for ind in 0..mods.len() {
        write.append(&mut mods[ind]);
        write.push(32);
        write.append(&mut names[ind]);
        write.push(0);
        write.append(&mut hashes[ind]);
    }

    println!("{:?}, {:?}", String::from_utf8_lossy(&write), mods.len());
    print!("{:?}", compute_sha1_hash_str(&String::from_utf8_lossy(&write)));
    // println!("{:?}", names);

    Ok(())
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
