use std::collections::HashMap;
use std::time::{Duration, Instant};

struct CrewAgentExecutor {
    i18n: I18N,
    iterations: i32,
    request_within_rpm_limit: Option<fn() -> bool>,
    max_iterations: Option<i32>,
    force_answer_max_iterations: Option<i32>,
}

impl CrewAgentExecutor {
    fn new() -> Self {
        CrewAgentExecutor {
            i18n: I18N::new(),
            iterations: 0,
            request_within_rpm_limit: None,
            max_iterations: Some(15),
            force_answer_max_iterations: None,
        }
    }

    fn set_force_answer_max_iterations(&mut self, values: &mut HashMap<String, i32>) {
        values.insert(
            "force_answer_max_iterations".to_string(),
            values["max_iterations"] - 2,
        );
    }

    fn should_force_answer(&self) -> bool {
        self.iterations == self.force_answer_max_iterations.unwrap()
    }

    fn force_answer(&self, output: AgentAction) -> AgentStep {
        AgentStep {
            action: output,
            observation: self.i18n.errors("used_too_many_tools"),
        }
    }

    fn call(
        &mut self,
        inputs: HashMap<String, String>,
        run_manager: Option<CallbackManagerForChainRun>,
    ) -> HashMap<String, Any> {
        let name_to_tool_map: HashMap<String, BaseTool> = self
            .tools
            .iter()
            .map(|tool| (tool.name.clone(), tool.clone()))
            .collect();
        let color_mapping = get_color_mapping(
            self.tools.iter().map(|tool| tool.name.clone()).collect(),
            vec!["green".to_string(), "red".to_string()],
        );
        let mut intermediate_steps: Vec<(AgentAction, String)> = vec![];
        self.iterations = 0;
        let start_time = Instant::now();
        let mut time_elapsed = Duration::from_secs(0);

        while self.should_continue(self.iterations, time_elapsed) {
            if self.request_within_rpm_limit.is_none()
                || self.request_within_rpm_limit.unwrap()()
            {
                let next_step_output = self.take_next_step(
                    &name_to_tool_map,
                    &color_mapping,
                    &inputs,
                    &mut intermediate_steps,
                    run_manager.clone(),
                );
                if let AgentFinish(output) = next_step_output {
                    return self.return_output(output, intermediate_steps, run_manager);
                }

                intermediate_steps.extend(next_step_output);
                if next_step_output.len() == 1 {
                    let next_step_action = &next_step_output[0];
                    let tool_return = self.get_tool_return(next_step_action);
                    if let Some(tool_return) = tool_return {
                        return self.return_output(tool_return, intermediate_steps, run_manager);
                    }
                }

                self.iterations += 1;
                time_elapsed = start_time.elapsed();
            }
        }

        let output = self.agent.return_stopped_response(
            self.early_stopping_method,
            intermediate_steps,
            inputs,
        );
        self.return_output(output, intermediate_steps, run_manager)
    }

    fn iter_next_step(
        &mut self,
        name_to_tool_map: &HashMap<String, BaseTool>,
        color_mapping: &HashMap<String, String>,
        inputs: &HashMap<String, String>,
        intermediate_steps: &mut Vec<(AgentAction, String)>,
        run_manager: Option<CallbackManagerForChainRun>,
    ) -> Vec<AgentStep> {
        let mut output = self.agent.plan(
            intermediate_steps,
            run_manager.as_ref().map(|rm| rm.get_child()),
            inputs,
        );

        if self.should_force_answer() {
            if let AgentAction(output) = output {
                return vec![self.force_answer(output)];
            } else if let CacheHit { action, .. } = output {
                return vec![self.force_answer(action)];
            } else {
                panic!("Unexpected output type from agent");
            }
        }

        let mut actions: Vec<AgentAction> = vec![];
        if let AgentAction(action) = output {
            actions.push(action);
        } else {
            actions = output;
        }

        let mut agent_steps: Vec<AgentStep> = vec![];
        for agent_action in actions {
            if let Some(run_manager) = &run_manager {
                run_manager.on_agent_action(&agent_action, "green");
            }

            if let Some(tool) = name_to_tool_map.get(&agent_action.tool) {
                let return_direct = tool.return_direct;
                let color = color_mapping.get(&agent_action.tool).unwrap();
                let mut tool_run_kwargs = self.agent.tool_run_logging_kwargs();
                if return_direct {
                    tool_run_kwargs.insert("llm_prefix", "");
                }

                let observation = tool.run(
                    &agent_action.tool_input,
                    self.verbose,
                    color,
                    run_manager.as_ref().map(|rm| rm.get_child()),
                    &tool_run_kwargs,
                );

                agent_steps.push(AgentStep {
                    action: agent_action,
                    observation,
                });
            } else {
                let mut tool_run_kwargs = self.agent.tool_run_logging_kwargs();
                let observation = InvalidTool().run(
                    &HashMap::new(),
                    self.verbose,
                    None,
                    run_manager.as_ref().map(|rm| rm.get_child()),
                    &tool_run_kwargs,
                );

                agent_steps.push(AgentStep {
                    action: agent_action,
                    observation,
                });
            }
        }

        agent_steps
    }
}
