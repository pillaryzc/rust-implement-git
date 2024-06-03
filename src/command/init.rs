// 命令接口
pub trait Command {
    fn execute(&self);
}

// 初始化命令
pub struct InitCommand {
    directory: String,
}

impl InitCommand {
    pub fn new(directory: String) -> Self {
        InitCommand { directory }
    }

    fn create_git_directory(&self) {
        std::fs::create_dir_all(format!("{}/.git", self.directory)).unwrap();
    }

    fn create_default_files(&self) {
        std::fs::write(format!("{}/.git/HEAD", self.directory), "ref: refs/heads/master\n").unwrap();
        std::fs::write(format!("{}/.git/config", self.directory), "[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n").unwrap();
        std::fs::write(format!("{}/.git/description", self.directory), "Unnamed repository; edit this file 'description' to name the repository.\n").unwrap();
        std::fs::create_dir_all(format!("{}/.git/hooks", self.directory)).unwrap();
        std::fs::create_dir_all(format!("{}/.git/info", self.directory)).unwrap();
        std::fs::write(format!("{}/.git/info/exclude", self.directory), "# git ls-files --others --exclude-from=.git/info/exclude\n# Lines that start with '#' are comments.\n").unwrap();
        std::fs::create_dir_all(format!("{}/.git/refs/heads", self.directory)).unwrap();
        std::fs::create_dir_all(format!("{}/.git/refs/tags", self.directory)).unwrap();
        std::fs::create_dir_all(format!("{}/.git/objects", self.directory)).unwrap();
    }
}

impl Command for InitCommand {
    fn execute(&self) {
        self.create_git_directory();
        self.create_default_files();
        println!("Initialized empty Git repository in {}/.git", self.directory);
    }
}