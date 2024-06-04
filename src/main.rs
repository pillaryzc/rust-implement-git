
use std::env;

use command::cat_file::CatFileCommand;
use command::init::InitCommand;
use command::Command;
use command::unkown::UnknownCommand;


pub mod command;
pub mod builder;
pub mod facade;


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // // Uncomment this block to pass the first stage
    // let args: Vec<String> = env::args().collect();
    // if args[1] == "init" {
    //     fs::create_dir(".git").unwrap();
    //     fs::create_dir(".git/objects").unwrap();
    //     fs::create_dir(".git/refs").unwrap();
    //     fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    //     println!("Initialized git directory")
    // }else if args[1] == "cat-file" {
    //     if args[2] == "-p"{
    //         let hash = args[3].as_str();
    //         dbg!(hash);
    //     }
    // }else {
    //     println!("unknown command: {}", args[1])
    // }
    let args: Vec<String> = env::args().collect();
    let command: Box<dyn Command> = match args[1].as_str() {
        "init" => Box::new(InitCommand),
        "cat-file" => Box::new(CatFileCommand),
        _ => Box::new(UnknownCommand),
    };
    command.execute(args);
}
