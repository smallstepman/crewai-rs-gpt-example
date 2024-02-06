use std::collections::HashMap;

struct I18N {
    // Implement the I18N struct
}

struct Prompts {
    i18n: I18N,
}

impl Prompts {
    const SCRATCHPAD_SLICE: &'static str = "\n{agent_scratchpad}";

    fn task_execution_with_memory(&self) -> BasePromptTemplate {
        self._build_prompt(vec!["role_playing", "tools", "memory", "task"])
    }

    fn task_execution_without_tools(&self) -> BasePromptTemplate {
        self._build_prompt(vec!["role_playing", "task"])
    }

    fn task_execution(&self) -> BasePromptTemplate {
        self._build_prompt(vec!["role_playing", "tools", "task"])
    }

    fn _build_prompt(&self, components: Vec<&str>) -> BasePromptTemplate {
        let mut prompt_parts: Vec<String> = components
            .iter()
            .map(|component| self.i18n.slice(component).to_owned())
            .collect();
        prompt_parts.push(Self::SCRATCHPAD_SLICE.to_owned());
        PromptTemplate::from_template(prompt_parts.join(""))
    }
}
