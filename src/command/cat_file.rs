use std::fs;
use std::io::Read;
use flate2::read::ZlibDecoder;

use super::Command;

pub struct CatFileCommand;
impl Command for CatFileCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() < 4 {
            println!("Invalid number of arguments");
            return;
        }
        let option = args[2].clone();
        let object_hash = args[3].clone();
        
        
        let object_path = format!(".git/objects/{}/{}",&object_hash[0..2], &object_hash[2..]);
        let compressed_data = fs::read(object_path).expect("Failed to read object file");
        let mut decoder = ZlibDecoder::new(&compressed_data[..]);
        let mut decoded_data = Vec::new();
        decoder.read_to_end(&mut decoded_data).expect("Failed to decode object");

        let null_index = decoded_data.iter().position(|&x| x == 0).expect("Invalid object format");
        let (header, contents) = decoded_data.split_at(null_index);
        let header_str = String::from_utf8_lossy(header);
        let contents_str = String::from_utf8_lossy(&contents[1..]);

        let object_type = header_str.split_whitespace().next().expect("Invalid object header");

        match option.as_str() {
            "-t" => {
                println!("{}", object_type);
            },
            "-p" => {
                println!("{}", contents_str);
            },
            _ => {
                println!("Invalid option");
            }
        }
    }
}