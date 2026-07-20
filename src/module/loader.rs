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
    pub fn resolve_from_directory(
       &self,
       directory: &std::path::Path,
       module: &str,
    ) -> std::path::PathBuf {

       let mut path = directory.to_path_buf();

       path.push(format!("{}.gn", module));

       path
    }
    pub fn module_exists_from_directory(
       &self,
       directory: &std::path::Path,
       module: &str,
    ) -> bool {

       let path = self.resolve_from_directory(
          directory,
          module,
       );

       path.exists()
    }
    pub fn load_source_from_directory(
       &self,
       directory: &std::path::Path,
       module: &str,
    ) -> String {

       let path =
          self.resolve_from_directory(
              directory,
              module,
          );

       match std::fs::read_to_string(&path) {

           Ok(source) => source,

           Err(error) => {

               panic!(
                  "Failed to load module '{}': {}",
                  module,
                  error,
               );

           }

       }
    }
    pub fn load_source(&self, module: &str) -> String {
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
    pub fn debug_load(&self, module: &str) {
       let source = self.load_source(module);

       println!("{}", source);
    }
}

