use crate::facade::GitFacade;

// GitRepositoryBuilder
pub struct GitRepositoryBuilder {
    directory: String,
}

impl GitRepositoryBuilder {
    pub fn new() -> Self {
        GitRepositoryBuilder {
            directory: ".".to_string(),
        }
    }

    pub fn directory(mut self, directory: String) -> Self {
        self.directory = directory;
        self
    }

    pub fn build(self) -> GitFacade {
        GitFacade::new(self.directory)
    }
}
