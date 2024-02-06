use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
enum Process {
    Sequential,
    Hierarchical,
}

#[derive(Debug, Deserialize, Serialize)]
struct Agent {
    role: String,
    goal: String,
    backstory: String,
    tools: Vec<String>,
    verbose: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    description: String,
    agent: Option<Agent>,
    async_execution: bool,
    tools: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Crew {
    tasks: Vec<Task>,
    agents: Vec<Agent>,
    process: Process,
    verbose: bool,
    config: Option<HashMap<String, serde_json::Value>>,
    max_rpm: Option<u32>,
    language: String,
}

impl Crew {
    fn kickoff(&self) -> String {
        // Initialize agents with language
        for agent in &self.agents {
            agent.i18n = I18N { language: self.language.clone() };
        }

        if self.process == Process::Sequential {
            return self.run_sequential_process();
        } else if self.process == Process::Hierarchical {
            return self.run_hierarchical_process();
        } else {
            panic!("The process '{:?}' is not implemented yet.", self.process);
        }
    }

    fn run_sequential_process(&self) -> String {
        let mut task_output = String::new();
        for task in &self.tasks {
            if let Some(agent) = &task.agent {
                if agent.allow_delegation {
                    let agents_for_delegation: Vec<&Agent> = self
                        .agents
                        .iter()
                        .filter(|a| a.role != agent.role)
                        .collect();
                    task.tools.extend(AgentTools::new(agents_for_delegation).tools());
                }
            }

            let role = task.agent.as_ref().map(|a| &a.role).unwrap_or("None");
            self.logger.log("debug", format!("Working Agent: {}", role));
            self.logger.log("info", format!("Starting Task: {}", task.description));

            let output = task.execute(&task_output);
            if !task.async_execution {
                task_output = output;
            }

            let role = task.agent.as_ref().map(|a| &a.role).unwrap_or("None");
            self.logger
                .log("debug", format!("[{}] Task output: {}\n\n", role, task_output));
        }

        if let Some(max_rpm) = self.max_rpm {
            self.rpm_controller.stop_rpm_counter();
        }

        task_output
    }

    fn run_hierarchical_process(&self) -> String {
        let i18n = I18N { language: self.language.clone() };
        let manager = Agent {
            role: i18n.retrieve("hierarchical_manager_agent", "role"),
            goal: i18n.retrieve("hierarchical_manager_agent", "goal"),
            backstory: i18n.retrieve("hierarchical_manager_agent", "backstory"),
            tools: AgentTools::new(&self.agents).tools(),
            verbose: true,
        };

        let mut task_output = String::new();
        for task in &self.tasks {
            self.logger.log("debug", format!("Working Agent: {}", manager.role));
            self.logger.log("info", format!("Starting Task: {}", task.description));

            task_output = task.execute(&manager, &task_output, &manager.tools);

            self.logger
                .log("debug", format!("[{}] Task output: {}\n\n", manager.role, task_output));
        }

        if let Some(max_rpm) = self.max_rpm {
            self.rpm_controller.stop_rpm_counter();
        }

        task_output
    }
}

fn main() {
    // Deserialize the Crew instance from JSON
    let crew_json = r#"
        {
            "tasks": [
                {
                    "description": "Task 1",
                    "agent": {
                        "role": "Agent 1",
                        "goal": "Goal 1",
                        "backstory": "Backstory 1",
                        "tools": ["Tool 1", "Tool 2"],
                        "verbose": true
                    },
                    "async_execution": false,
                    "tools": ["Tool 3", "Tool 4"]
                },
                {
                    "description": "Task 2",
                    "agent": null,
                    "async_execution": true,
                    "tools": ["Tool 5", "Tool 6"]
                }
            ],
            "agents": [
                {
                    "role": "Agent 1",
                    "goal": "Goal 1",
                    "backstory": "Backstory 1",
                    "tools": ["Tool 1", "Tool 2"],
                    "verbose": true
                },
                {
                    "role": "Agent 2",
                    "goal": "Goal 2",
                    "backstory": "Backstory 2",
                    "tools": ["Tool 3", "Tool 4"],
                    "verbose": false
                }
            ],
            "process": "Sequential",
            "verbose": true,
            "config": null,
            "max_rpm": 100,
            "language": "en"
        }
    "#;

    let crew: Crew = serde_json::from_str(crew_json).unwrap();

    // Start the crew
    let output = crew.kickoff();
    println!("Crew output: {}", output);
}
