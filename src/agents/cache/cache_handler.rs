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
        let key = format!("{}-{}", tool, input.trim());
        self.cache.insert(key, output.to_string());
    }

    fn read(&self, tool: &str, input: &str) -> Option<&str> {
        let key = format!("{}-{}", tool, input.trim());
        self.cache.get(&key).map(|s| s.as_str())
    }
}

fn main() {
    let mut cache_handler = CacheHandler::new();
    cache_handler.add("tool1", "input1", "output1");
    cache_handler.add("tool2", "input2", "output2");

    let result1 = cache_handler.read("tool1", "input1");
    let result2 = cache_handler.read("tool2", "input2");
    let result3 = cache_handler.read("tool3", "input3");

    println!("{:?}", result1);
    println!("{:?}", result2);
    println!("{:?}", result3);
}
