use std::fs;

use super::Command;

// 初始化命令
pub(crate) struct InitCommand;
impl Command for InitCommand {
    fn execute(&self, args: Vec<String>) {
        let _ = args;
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    }
}