use std::error::Error;
use std::fmt;

struct TaskRepeatedUsageException {
    i18n: I18N,
    error: String,
    message: String,
    tool: String,
    tool_input: String,
    text: String,
}

impl TaskRepeatedUsageException {
    fn new(i18n: I18N, tool: String, tool_input: String, text: String) -> Self {
        let message = i18n.errors("task_repeated_usage").format(
            tool=tool,
            tool_input=tool_input,
        );
        
        TaskRepeatedUsageException {
            i18n,
            error: String::from("TaskRepeatedUsageException"),
            message,
            tool,
            tool_input,
            text,
        }
    }
}

impl fmt::Display for TaskRepeatedUsageException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TaskRepeatedUsageException {}
