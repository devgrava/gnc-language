#[derive(Debug, Default)]
pub struct ModuleContext {
    loaded_modules: Vec<String>,
}

impl ModuleContext {
    pub fn new() -> Self {
        Self {
            loaded_modules: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        module: String,
    ) {
        self.loaded_modules.push(module);
    }

    pub fn contains(
        &self,
        module: &str,
    ) -> bool {
        self.loaded_modules.iter().any(|m| m == module)
    }

    pub fn clear(&mut self) {
        self.loaded_modules.clear();
    }
}
