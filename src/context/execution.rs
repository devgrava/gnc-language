use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    current_file: Option<PathBuf>,
    current_directory: Option<PathBuf>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            current_file: None,
            current_directory: None,
        }
    }

    pub fn set_current_file(
        &mut self,
        path: PathBuf,
    ) {
        self.current_directory =
            path.parent().map(|p| p.to_path_buf());

        self.current_file = Some(path);
    }

    pub fn current_file(&self) -> Option<&PathBuf> {
        self.current_file.as_ref()
    }

    pub fn current_directory(&self) -> Option<&PathBuf> {
        self.current_directory.as_ref()
    }

    pub fn clear(&mut self) {
        self.current_file = None;
        self.current_directory = None;
    }
}
