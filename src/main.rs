
use std::env;

use command::cat_file::CatFileCommand;
use command::hash_object::HashObjectCommand;
use command::init::InitCommand;
use command::Command;
use command::unkown::UnknownCommand;


pub mod command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: Box<dyn Command> = match args[1].as_str() {
        "init" => Box::new(InitCommand),
        "cat-file" => Box::new(CatFileCommand),
        "hash-object" => Box::new(HashObjectCommand),
        _ => Box::new(UnknownCommand),
    };
    command.execute(args);
}
