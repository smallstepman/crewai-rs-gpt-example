use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
struct TaskOutput {
    #[serde(rename = "description")]
    #[validate(length(min = 1))]
    description: String,

    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,

    #[serde(rename = "result")]
    #[validate(length(min = 1))]
    result: String,
}

impl TaskOutput {
    fn set_summary(&mut self) {
        let excerpt = self.description.split(" ").take(10).collect::<Vec<&str>>().join(" ");
        self.summary = Some(format!("{}...", excerpt));
    }
}
