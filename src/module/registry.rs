pub struct ModuleRegistry;

impl ModuleRegistry {
    pub fn exists(name: &str) -> bool {
        matches!(
            name,
            "math"
                | "string"
                | "array"
                | "dictionary"
                | "io"
                | "system"
        )
    }
}
