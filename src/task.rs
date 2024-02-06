use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

#[pyclass]
struct I18N;

#[pyclass]
struct TaskOutput {
    description: String,
    result: String,
}

#[pyclass]
struct Agent {
    tools: Vec<String>,
    // Other fields and methods as required
}

#[pyclass]
#[derive(Clone)]
struct Task {
    #[pyo3(get)]
    description: String,
    callback: Option<PyObject>,
    agent: Option<Agent>,
    expected_output: Option<String>,
    context: Option<Vec<Task>>,
    async_execution: Option<bool>,
    output: Option<TaskOutput>,
    tools: Vec<String>,
    id: Uuid,
}

#[pymethods]
impl Task {
    #[new]
    fn new(description: String) -> Self {
        Task {
            description,
            callback: None,
            agent: None,
            expected_output: None,
            context: None,
            async_execution: None,
            output: None,
            tools: vec![],
            id: Uuid::new_v4(),
        }
    }

    fn execute(&self, agent: Option<&Agent>, context: Option<&str>, tools: Option<Vec<String>>) -> PyResult<String> {
        let agent = agent.or(self.agent.as_ref());
        if agent.is_none() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                "The task has no agent assigned, therefore it can't be executed directly.",
            ));
        }

        let agent = agent.unwrap();
        let context = match &self.context {
            Some(tasks) => {
                let mut context_results = vec![];
                for task in tasks {
                    if task.async_execution.unwrap_or(false) {
                        // Join the thread if it's async (not implemented here)
                    }
                    if let Some(output) = &task.output {
                        context_results.push(output.result.clone());
                    }
                }
                Some(context_results.join("\n"))
            }
            None => context.map(|s| s.to_string()),
        };

        let tools = tools.or_else(|| Some(self.tools.clone()));

        if self.async_execution.unwrap_or(false) {
            // Execute asynchronously (not implemented here)
        } else {
            let result = self._execute(agent, &self._prompt(), context.as_deref(), &tools.unwrap());
            return Ok(result);
        }

        Ok("".to_string())
    }

    fn _execute(&self, agent: &Agent, task_prompt: &str, context: Option<&str>, tools: &[String]) -> String {
        // Execute the task using the agent (not implemented here)
        let result = format!("Executed task with prompt: {}", task_prompt);
        if let Some(callback) = &self.callback {
            // Invoke the callback with the result (not implemented here)
        }
        result
    }

    fn _prompt(&self) -> String {
        let mut tasks_slices = vec![self.description.clone()];

        if let Some(expected_output) = &self.expected_output {
            // Use I18N to format the expected output (not implemented here)
            let output = format!("Expected output: {}", expected_output);
            tasks_slices.push(output);
        }

        tasks_slices.join("\n")
    }
}

#[pymodule]
fn crewai_task_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Task>()?;
    Ok(())
}
