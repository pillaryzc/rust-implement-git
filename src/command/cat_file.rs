use std::fs;
use std::io::Read;
use flate2::read::ZlibDecoder;

pub struct CatFileCommand {
    directory: String,
    object_hash: String,
    option: String,
}

impl CatFileCommand {
    pub fn new(directory: String, object_hash: String, option: String) -> Self {
        CatFileCommand { directory, object_hash, option }
    }

    pub fn execute(&self) {
        let object_path = format!("{}/.git/objects/{}/{}", self.directory, &self.object_hash[0..2], &self.object_hash[2..]);
        let compressed_data = fs::read(object_path).expect("Failed to read object file");
        let mut decoder = ZlibDecoder::new(&compressed_data[..]);
        let mut decoded_data = Vec::new();
        decoder.read_to_end(&mut decoded_data).expect("Failed to decode object");

        let null_index = decoded_data.iter().position(|&x| x == 0).expect("Invalid object format");
        let (header, contents) = decoded_data.split_at(null_index);
        let header_str = String::from_utf8_lossy(header);
        let contents_str = String::from_utf8_lossy(&contents[1..]);

        let object_type = header_str.split_whitespace().next().expect("Invalid object header");

        match self.option.as_str() {
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
