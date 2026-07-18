use std::collections::HashSet;
use std::path::PathBuf;

pub struct ModuleLoader {
    loaded: HashSet<String>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            loaded: HashSet::new(),
        }
    }

    pub fn is_loaded(&self, module: &str) -> bool {
        self.loaded.contains(module)
    }

    pub fn load(&mut self, module: &str) {
        self.loaded.insert(module.to_string());
    }
    pub fn is_builtin(module: &str) -> bool {
       matches!(
          module,
          "math"
            | "string"
            | "array"
            | "dictionary"
            | "io"
            | "system"
       )
    }
    pub fn resolve_path(module: &str) -> PathBuf {
          let mut path = PathBuf::new();

          path.push(format!("{}.gn", module));

          path
    }
}
