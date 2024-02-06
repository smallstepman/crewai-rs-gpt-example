use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct CacheHandler {
    // Assuming CacheHandler has some fields and methods relevant to caching
}

struct ToolsHandler {
    last_used_tool: Arc<Mutex<HashMap<String, String>>>,
    cache: Arc<Mutex<CacheHandler>>,
}

impl ToolsHandler {
    fn new(cache: Arc<Mutex<CacheHandler>>) -> Self {
        ToolsHandler {
            last_used_tool: Arc::new(Mutex::new(HashMap::new())),
            cache,
        }
    }

    fn on_tool_start(&self, serialized: &HashMap<String, String>, input_str: &str) {
        let name = serialized.get("name").unwrap_or(&"".to_string());
        if name != "invalid_tool" && name != "_Exception" {
            let mut tools_usage = self.last_used_tool.lock().unwrap();
            tools_usage.insert("tool".to_string(), name.to_string());
            tools_usage.insert("input".to_string(), input_str.to_string());
        }
    }

    fn on_tool_end(&self, output: &str) {
        if !output.contains("is not a valid tool")
            && !output.contains("Invalid or incomplete response")
            && !output.contains("Invalid Format")
        {
            let tools_usage = self.last_used_tool.lock().unwrap();
            let tool_name = tools_usage.get("tool").unwrap_or(&"".to_string());
            let tool_input = tools_usage.get("input").unwrap_or(&"".to_string());

            // Assuming CacheHandler has an `add` method
            let mut cache = self.cache.lock().unwrap();
            cache.add(tool_name, tool_input, output);
        }
    }
}

// Assuming CacheTools has a `name` method that returns the name of the tool
struct CacheTools;

impl CacheTools {
    fn name() -> String {
        // Placeholder for the actual implementation
        "cache_tool_name".to_string()
    }
}
