pub mod cat_file;
pub mod init;
pub mod unkown;
pub trait Command {
    fn execute(&self, args: Vec<String>);
}
