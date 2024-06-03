use crate::command::init::Command;

// GitFacade 门面类
pub struct GitFacade {
    init_command: Box<dyn Command>,
}

impl GitFacade {
    pub fn new(directory: String) -> Self {
        GitFacade {
            init_command: Box::new(crate::command::init::InitCommand::new(directory)),
        }
    }

    pub fn init(&self) {
        self.init_command.execute();
    }
}
