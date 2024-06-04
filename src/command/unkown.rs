use super::Command;


pub(crate) struct  UnknownCommand;
impl Command for UnknownCommand {
    fn execute(&self, args: Vec<String>) {
        println!("unknown command: {}", args[1])
    }
}