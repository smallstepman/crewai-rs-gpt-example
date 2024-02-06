use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct CacheHandler {
    cache: Mutex<HashMap<String, String>>,
}

impl CacheHandler {
    fn new() -> Self {
        CacheHandler {
            cache: Mutex::new(HashMap::new()),
        }
    }

    fn read(&self, tool: &str, tool_input: &str) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        cache.get(&format!("{}:{}", tool, tool_input)).cloned()
    }
}

#[derive(Serialize, Deserialize)]
struct CacheTools {
    #[serde(rename = "modelConfig")]
    model_config: HashMap<String, serde_json::Value>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_handler: Option<CacheHandler>,
}

impl CacheTools {
    fn new() -> Self {
        CacheTools {
            model_config: HashMap::new(),
            name: "Hit Cache".to_string(),
            cache_handler: Some(CacheHandler::new()),
        }
    }

    fn tool(&self) -> Tool {
        Tool {
            name: self.name.clone(),
            description: "Reads directly from the cache".to_string(),
            func: Box::new(|key: &str| {
                let split: Vec<&str> = key.split("tool:").collect();
                let tool = split[1].split("|input:").next().unwrap().trim();
                let tool_input = split[1].split("|input:").nth(1).unwrap().trim();
                self.cache_handler.as_ref().unwrap().read(tool, tool_input)
            }),
        }
    }
}

struct Tool {
    name: String,
    description: String,
    func: Box<dyn Fn(&str) -> Option<String>>,
}
