pub mod cat_file;
pub mod init;
pub mod unkown;
pub mod hash_object;

pub trait Command {
    fn execute(&self, args: Vec<String>);
}
