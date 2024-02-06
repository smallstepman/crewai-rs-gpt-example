use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TaskOutput {
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    result: String,
}

#[pymethods]
impl TaskOutput {
    #[new]
    fn new(description: String, result: String) -> Self {
        let summary = description.split_whitespace().take(10).collect::<Vec<&str>>().join(" ");
        TaskOutput {
            description,
            summary: Some(format!("{}...", summary)),
            result,
        }
    }

    #[getter]
    fn get_description(&self) -> PyResult<String> {
        Ok(self.description.clone())
    }

    #[setter]
    fn set_description(&mut self, description: String) -> PyResult<()> {
        self.description = description;
        Ok(())
    }

    #[getter]
    fn get_summary(&self) -> PyResult<Option<String>> {
        Ok(self.summary.clone())
    }

    #[setter]
    fn set_summary(&mut self, summary: Option<String>) -> PyResult<()> {
        self.summary = summary;
        Ok(())
    }

    #[getter]
    fn get_result(&self) -> PyResult<String> {
        Ok(self.result.clone())
    }

    #[setter]
    fn set_result(&mut self, result: String) -> PyResult<()> {
        self.result = result;
        Ok(())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn task_output_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TaskOutput>()?;
    Ok(())
}
