#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::path::PathBuf;
use std::io::Read;
extern crate flate2;
use flate2::read::ZlibDecoder;

fn read_cat_file(sha: &String){

    let sha_file_path = PathBuf::from(format!(".git/objects/{}/{}", &sha[0..2], &sha[2..]));
    let sha_file = fs::File::open(&sha_file_path);
    let mut decoder= ZlibDecoder::new(sha_file.unwrap());
    let mut decompressed_data = String::new();
    let _ = decoder.read_to_string(&mut decompressed_data);
    print!("{}", decompressed_data);
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

    else {
        println!("unknown command: {}", args[1])
    }



}
