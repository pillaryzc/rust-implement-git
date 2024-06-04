use std::fs;
use std::io::Write;
use std::path::Path;
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;

use super::Command;

pub struct HashObjectCommand;
impl Command for HashObjectCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() < 3 {
            println!("Invalid number of arguments");
            return;
        }
        let file_path = &args[2];
        let data = fs::read(file_path).expect("Failed to read file");
        let hash = Sha1::digest(&data);
        let hash_string = format!("{:x}", hash);

        let object_dir = format!(".git/objects/{}/", &hash_string[0..2]);
        fs::create_dir_all(&object_dir).expect("Failed to create object directory");

        let object_path = format!("{}{}", object_dir, &hash_string[2..]);
        if Path::new(&object_path).exists() {
            println!("{}", hash_string);
            return;
        }

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(b"blob ").expect("Failed to write to buffer");
        encoder.write_all(data.len().to_string().as_bytes()).expect("Failed to write to buffer");
        encoder.write_all(b"\0").expect("Failed to write to buffer");
        encoder.write_all(&data).expect("Failed to write to buffer");
        let compressed = encoder.finish().expect("Failed to finish compression");

        fs::write(&object_path, compressed).expect("Failed to write object file");
        println!("{}", hash_string);
    }
}