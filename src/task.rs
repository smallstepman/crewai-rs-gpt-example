use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

struct Task {
    description: String,
    callback: Option<Box<dyn Fn(TaskOutput)>>,
    agent: Option<Agent>,
    expected_output: Option<String>,
    context: Option<Vec<Task>>,
    async_execution: bool,
    output: Option<TaskOutput>,
    tools: Vec<Box<dyn Any>>,
    id: Uuid,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            callback: None,
            agent: None,
            expected_output: None,
            context: None,
            async_execution: false,
            output: None,
            tools: Vec::new(),
            id: Uuid::new_v4(),
        }
    }

    fn execute(
        &mut self,
        agent: Option<Agent>,
        context: Option<String>,
        tools: Option<Vec<Box<dyn Any>>>,
    ) -> String {
        let agent = agent.or(self.agent.clone()).expect("The task has no agent assigned.");
        
        let context = if let Some(context_tasks) = &self.context {
            let mut context_output = Vec::new();
            for task in context_tasks {
                if task.async_execution {
                    task.thread.join().expect("Failed to join thread.");
                }
                if let Some(output) = &task.output {
                    context_output.push(output.result.clone());
                }
            }
            Some(context_output.join("\n"))
        } else {
            context
        };
        
        let tools = tools.unwrap_or_else(|| self.tools.clone());
        
        if self.async_execution {
            let task = self.clone();
            self.thread = Some(thread::spawn(move || {
                task._execute(agent, task._prompt(), context, tools)
            }));
            String::new()
        } else {
            self._execute(agent, self._prompt(), context, tools)
        }
    }
    
    fn _execute(&mut self, agent: Agent, task_prompt: String, context: Option<String>, tools: Vec<Box<dyn Any>>) -> String {
        let result = agent.execute_task(task_prompt, context, tools);
        self.output = Some(TaskOutput {
            description: self.description.clone(),
            result: result.clone(),
        });
        if let Some(callback) = &self.callback {
            callback(self.output.clone().unwrap());
        }
        result
    }
    
    fn _prompt(&self) -> String {
        let mut task_slices = vec![self.description.clone()];
        
        if let Some(expected_output) = &self.expected_output {
            let output = format!("Expected Output: {}", expected_output);
            task_slices.push(output);
        }
        
        task_slices.join("\n")
    }
}

fn main() {
    let mut task = Task::new("Task Description".to_string());
    task.execute(None, None, None);
}
