use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct I18N {
    translations: HashMap<String, String>,
}

impl I18N {
    fn slice(&self, key: &str) -> String {
        self.translations.get(key).cloned().unwrap_or_default()
    }
}

#[derive(Serialize, Deserialize)]
struct Prompts {
    i18n: I18N,
}

impl Prompts {
    const SCRATCHPAD_SLICE: &'static str = "\n{agent_scratchpad}";

    fn task_execution_with_memory(&self) -> String {
        self.build_prompt(vec!["role_playing", "tools", "memory", "task"])
    }

    fn task_execution_without_tools(&self) -> String {
        self.build_prompt(vec!["role_playing", "task"])
    }

    fn task_execution(&self) -> String {
        self.build_prompt(vec!["role_playing", "tools", "task"])
    }

    fn build_prompt(&self, components: Vec<&str>) -> String {
        let mut prompt_parts: Vec<String> = components
            .iter()
            .map(|component| self.i18n.slice(component))
            .collect();
        prompt_parts.push(String::from(Self::SCRATCHPAD_SLICE));
        prompt_parts.join("")
    }
}
