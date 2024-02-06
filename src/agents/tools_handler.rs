use std::collections::HashMap;

struct CacheHandler {}

struct ToolsHandler {
    last_used_tool: HashMap<String, serde_json::Value>,
    cache: CacheHandler,
}

impl ToolsHandler {
    fn new(cache: CacheHandler) -> Self {
        ToolsHandler {
            last_used_tool: HashMap::new(),
            cache,
        }
    }

    fn on_tool_start(&mut self, serialized: HashMap<String, serde_json::Value>, input_str: String) {
        let name = serialized.get("name").unwrap().as_str().unwrap();
        if name != "invalid_tool" && name != "_Exception" {
            let tools_usage = HashMap::new();
            tools_usage.insert("tool".to_string(), serde_json::Value::String(name.to_string()));
            tools_usage.insert("input".to_string(), serde_json::Value::String(input_str));
            self.last_used_tool = tools_usage;
        }
    }

    fn on_tool_end(&self, output: String) {
        if !output.contains("is not a valid tool")
            && !output.contains("Invalid or incomplete response")
            && !output.contains("Invalid Format")
        {
            if self.last_used_tool.get("tool").unwrap().as_str().unwrap() != CacheTools.name() {
                self.cache.add(
                    self.last_used_tool.get("tool").unwrap().as_str().unwrap(),
                    self.last_used_tool.get("input").unwrap().as_str().unwrap(),
                    output,
                );
            }
        }
    }
}
