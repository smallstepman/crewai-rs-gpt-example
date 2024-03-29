use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct HierarchicalManagerAgent {
    role: String,
    goal: String,
    backstory: String,
}

#[derive(Serialize, Deserialize)]
struct Slices {
    observation: String,
    task: String,
    memory: String,
    role_playing: String,
    tools: String,
    task_with_context: String,
    expected_output: String,
}

#[derive(Serialize, Deserialize)]
struct Errors {
    used_too_many_tools: String,
    agent_tool_missing_param: String,
    agent_tool_unexsiting_coworker: String,
    task_repeated_usage: String,
}

#[derive(Serialize, Deserialize)]
struct Tools {
    delegate_work: String,
    ask_question: String,
}

#[derive(Serialize, Deserialize)]
struct Root {
    hierarchical_manager_agent: HierarchicalManagerAgent,
    slices: Slices,
    errors: Errors,
    tools: Tools,
}

fn main() {
    let json = r#"
    {
        "hierarchical_manager_agent": {
            "role": "Crew Manager",
            "goal": "Manage the team to complete the task in the best way possible.",
            "backstory": "You are a seasoned manager with a knack for getting the best out of your team.\\nYou are also known for your ability to delegate work to the right people, and to ask the right questions to get the best out of your team.\\nEven though you don't perform tasks by yourself, you have a lot of experience in the field, which allows you to properly evaluate the work of your team members."
        },
        "slices": {
            "observation": "\\nObservation",
            "task": "Begin! This is VERY important to you, your job depends on it!\\n\\nCurrent Task: {input}",
            "memory": "This is the summary of your work so far:\\n{chat_history}",
            "role_playing": "You are {role}.\\n{backstory}\\n\\nYour personal goal is: {goal}",
            "tools": "TOOLS:\\n------\\nYou have access to only the following tools:\\n\\n{tools}\\n\\nTo use a tool, please use the exact following format:\\n\\n``\\nThought: Do I need to use a tool? Yes\\nAction: the action to take, should be one of [{tool_names}], just the name.\\nAction Input: the input to the action\\nObservation: the result of the action\\n``\\n\\nWhen you have a response for your task, or if you do not need to use a tool, you MUST use the format:\\n\\n``\\nThought: Do I need to use a tool? No\\nFinal Answer: [your response here]",
            "task_with_context": "{task}\\nThis is the context you're working with:\\n{context}",
            "expected_output": "Your final answer must be: {expected_output}"
        },
        "errors": {
            "used_too_many_tools": "I've used too many tools for this task. I'm going to give you my absolute BEST Final answer now and not use any more tools.",
            "agent_tool_missing_param": "\\nError executing tool. Missing exact 3 pipe (|) separated values. For example, `coworker|task|context`. I need to make sure to pass context as context.\\n",
            "agent_tool_unexsiting_coworker": "\\nError executing tool. Co-worker mentioned on the Action Input not found, it must to be one of the following options: {coworkers}.\\n",
            "task_repeated_usage": "I just used the {tool} tool with input {tool_input}. So I already know the result of that and don't need to use it now.\\n"
        },
        "tools": {
            "delegate_work": "Useful to delegate a specific task to one of the following co-workers: {coworkers}.\\nThe input to this tool should be a pipe (|) separated text of length 3 (three), representing the co-worker you want to ask it to (one of the options), the task and all actual context you have for the task.\\nFor example, `coworker|task|context`.",
            "ask_question": "Useful to ask a question, opinion or take from on of the following co-workers: {coworkers}.\\nThe input to this tool should be a pipe (|) separated text of length 3 (three), representing the co-worker you want to ask it to (one of the options), the question and all actual context you have for the question.\\n For example, `coworker|question|context`."
        }
    }
    "#;

    let parsed: Root = serde_json::from_str(json).unwrap();
    println!("{:?}", parsed);
}
