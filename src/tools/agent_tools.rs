use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Agent {
    role: String,
    // Other fields that define an Agent
}

#[derive(Serialize, Deserialize)]
struct I18N {
    // Define the structure for I18N
}

#[derive(Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    // Other fields that define a Tool
}

impl Tool {
    fn from_function(name: &str, description: &str) -> Self {
        Tool {
            name: name.to_string(),
            description: description.to_string(),
            // Initialize other fields
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AgentTools {
    agents: Vec<Agent>,
    i18n: I18N,
}

impl AgentTools {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool::from_function(
                "Delegate work to co-worker",
                &self.i18n.tools("delegate_work", &self.agent_roles()),
            ),
            Tool::from_function(
                "Ask question to co-worker",
                &self.i18n.tools("ask_question", &self.agent_roles()),
            ),
        ]
    }

    fn delegate_work(&self, command: &str) -> String {
        self.execute(command)
    }

    fn ask_question(&self, command: &str) -> String {
        self.execute(command)
    }

    fn execute(&self, command: &str) -> String {
        let parts: Vec<&str> = command.split('|').collect();
        if parts.len() != 3 {
            return self.i18n.errors("agent_tool_missing_param");
        }

        let (agent_role, task, context) = (parts[0], parts[1], parts[2]);
        if agent_role.is_empty() || task.is_empty() || context.is_empty() {
            return self.i18n.errors("agent_tool_missing_param");
        }

        let agent = self.agents.iter().find(|a| a.role == agent_role);
        match agent {
            Some(agent) => agent.execute_task(task, context),
            None => self.i18n.errors("agent_tool_unexisting_coworker", &self.agent_roles()),
        }
    }

    fn agent_roles(&self) -> String {
        self.agents.iter().map(|agent| &agent.role).collect::<Vec<&String>>().join(", ")
    }
}

impl I18N {
    fn tools(&self, key: &str, params: &str) -> String {
        // Implement the logic to return the internationalized string for tools
        format!("{}: {}", key, params)
    }

    fn errors(&self, key: &str, params: &str) -> String {
        // Implement the logic to return the internationalized string for errors
        format!("{}: {}", key, params)
    }
}

impl Agent {
    fn execute_task(&self, task: &str, context: &str) -> String {
        // Implement the logic to execute the task
        format!("Executing task: {} with context: {}", task, context)
    }
}
