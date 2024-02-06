use std::collections::HashMap;

struct CacheHandler {
    cache: HashMap<String, String>,
}

impl CacheHandler {
    fn new() -> CacheHandler {
        CacheHandler {
            cache: HashMap::new(),
        }
    }

    fn add(&mut self, tool: &str, input: &str, output: &str) {
        let input = input.trim();
        self.cache.insert(format!("{}-{}", tool, input), output.to_string());
    }

    fn read(&self, tool: &str, input: &str) -> Option<&String> {
        let input = input.trim();
        self.cache.get(&format!("{}-{}", tool, input))
    }
}
