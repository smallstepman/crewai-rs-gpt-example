use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskRepeatedUsageException {
    #[error("{0}")]
    TaskRepeatedUsageException(String),
}

pub struct I18N {
    // Assuming I18N struct and its methods are defined elsewhere
}

impl I18N {
    pub fn errors(&self, error_code: &str) -> String {
        // This method should return the error message template
        // based on the error_code. Implementation details are omitted.
        String::new()
    }
}

pub struct TaskRepeatedUsage {
    i18n: I18N,
    tool: String,
    tool_input: String,
    text: String,
    message: String,
}

impl TaskRepeatedUsage {
    pub fn new(i18n: I18N, tool: &str, tool_input: &str, text: &str) -> Self {
        let message = i18n.errors("task_repeated_usage")
            .replace("{tool}", tool)
            .replace("{tool_input}", tool_input);

        TaskRepeatedUsage {
            i18n,
            tool: tool.to_string(),
            tool_input: tool_input.to_string(),
            text: text.to_string(),
            message,
        }
    }

    pub fn to_error(&self) -> TaskRepeatedUsageException {
        TaskRepeatedUsageException::TaskRepeatedUsageException(self.message.clone())
    }
}

// Usage example:
// let i18n = I18N::new(); // Assuming a new method for I18N
// let task_repeated_usage = TaskRepeatedUsage::new(i18n, "ToolName", "ToolInput", "Some text");
// let error = task_repeated_usage.to_error();
