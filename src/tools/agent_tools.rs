use std::collections::HashMap;

struct Agent {
    role: String,
}

impl Agent {
    fn execute_task(&self, task: &str, context: &str) -> String {
        // Implement the logic to execute the task
        String::from("Task executed")
    }
}

struct I18N {
    // Implement the I18N struct
}

struct Tool {
    // Implement the Tool struct
}

struct AgentTools {
    agents: Vec<Agent>,
    i18n: I18N,
}

impl AgentTools {
    fn tools(&self) -> Vec<Tool> {
        let mut tools = Vec::new();

        let delegate_work_tool = Tool::from_function(
            self.delegate_work,
            "Delegate work to co-worker",
            format!(
                "{}",
                self.i18n.tools("delegate_work").format(
                    self.agents
                        .iter()
                        .map(|agent| agent.role.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            ),
        );
        tools.push(delegate_work_tool);

        let ask_question_tool = Tool::from_function(
            self.ask_question,
            "Ask question to co-worker",
            format!(
                "{}",
                self.i18n.tools("ask_question").format(
                    self.agents
                        .iter()
                        .map(|agent| agent.role.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            ),
        );
        tools.push(ask_question_tool);

        tools
    }

    fn delegate_work(&self, command: &str) -> String {
        self._execute(command)
    }

    fn ask_question(&self, command: &str) -> String {
        self._execute(command)
    }

    fn _execute(&self, command: &str) -> String {
        let parts: Vec<&str> = command.split("|").collect();
        if parts.len() != 3 {
            return self.i18n.errors("agent_tool_missing_param").to_string();
        }

        let agent = parts[0];
        let task = parts[1];
        let context = parts[2];

        if agent.is_empty() || task.is_empty() || context.is_empty() {
            return self.i18n.errors("agent_tool_missing_param").to_string();
        }

        let agent = self
            .agents
            .iter()
            .find(|available_agent| available_agent.role == agent);

        if let Some(agent) = agent {
            agent.execute_task(task, context)
        } else {
            self.i18n
                .errors("agent_tool_unexsiting_coworker")
                .format(
                    self.agents
                        .iter()
                        .map(|agent| agent.role.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
                .to_string()
        }
    }
}

fn main() {
    let agents = vec![
        Agent {
            role: String::from("Agent 1"),
        },
        Agent {
            role: String::from("Agent 2"),
        },
    ];
    let i18n = I18N {
        // Initialize the I18N struct
    };
    let agent_tools = AgentTools { agents, i18n };

    let tools = agent_tools.tools();
    for tool in tools {
        println!("Name: {}", tool.name);
        println!("Description: {}", tool.description);
    }
}
