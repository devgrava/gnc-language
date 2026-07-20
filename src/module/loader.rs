use std::collections::HashSet;
use std::fs;
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

    pub fn mark_loaded(&mut self, module: &str) {
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
    pub fn resolve_path(&self, module: &str) -> PathBuf {
          let mut path = PathBuf::new();

          path.push(format!("{}.gn", module));

          path
    }
    pub fn module_exists(&self, module: &str) -> bool {
       let path = self.resolve_path(module);

       path.exists()
    }
    pub fn load_source(module: &str) -> String {
       let path = PathBuf::from(format!("{}.gn", module));

       match fs::read_to_string(&path) {
            Ok(source) => source,

            Err(error) => {
               panic!(
                   "Failed to load module '{}': {}",
                   module,
                   error
               );
            }
       }
    }
    pub fn debug_load(module: &str) {
       let source = Self::load_source(module);

       println!("{}", source);
    }
}

