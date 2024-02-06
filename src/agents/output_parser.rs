use regex::Regex;
use std::collections::HashMap;

struct CrewAgentOutputParser {
    tools_handler: ToolsHandler,
    cache: CacheHandler,
    i18n: I18N,
}

impl CrewAgentOutputParser {
    fn parse(&self, text: &str) -> Result<AgentAction, AgentFinish, CacheHit> {
        let regex = Regex::new(r"Action\s*\d*\s*:[\s]*(.*?)[\s]*Action\s*\d*\s*Input\s*\d*\s*:[\s]*(.*)").unwrap();
        if let Some(captures) = regex.captures(text) {
            let action = captures.get(1).unwrap().as_str().trim();
            let action_input = captures.get(2).unwrap().as_str().trim();
            let tool_input = action_input.trim_matches(' ').trim_matches('"');

            if let Some(last_tool_usage) = self.tools_handler.last_used_tool {
                let usage = {
                    "tool": action,
                    "input": tool_input,
                };
                if usage == last_tool_usage {
                    return Err(TaskRepeatedUsageException {
                        text: text.to_string(),
                        tool: action.to_string(),
                        tool_input: tool_input.to_string(),
                        i18n: self.i18n.clone(),
                    });
                }
            }

            if let Some(action) = self.cache.read(action, tool_input) {
                return Ok(CacheHit {
                    action: AgentAction {
                        action: action.to_string(),
                        tool_input: tool_input.to_string(),
                        text: text.to_string(),
                    },
                    cache: self.cache.clone(),
                });
            }
        }

        super::parse(text)
    }
}
