use pyo3::prelude::*;
use pyo3::types::{PyDict, PyType};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[pyclass]
struct RPMController {
    model_config: PyObject,
    max_rpm: Option<i32>,
    logger: PyObject,
    current_rpm: Arc<Mutex<i32>>,
    timer: Option<PyObject>,
    lock: Arc<Mutex<()>>,
}

#[pymethods]
impl RPMController {
    #[new]
    fn new(model_config: PyObject, max_rpm: Option<i32>, logger: PyObject) -> Self {
        RPMController {
            model_config,
            max_rpm,
            logger,
            current_rpm: Arc::new(Mutex::new(0)),
            timer: None,
            lock: Arc::new(Mutex::new(())),
        }
    }

    #[setter]
    fn set_max_rpm(&mut self, value: Option<i32>) {
        self.max_rpm = value;
    }

    #[getter]
    fn get_max_rpm(&self) -> Option<i32> {
        self.max_rpm
    }

    fn reset_counter(&mut self, py: Python) {
        if let Some(max_rpm) = self.max_rpm {
            let lock = self.lock.clone();
            let current_rpm = self.current_rpm.clone();
            self.timer = Some(pyo3_asyncio::async_std::future_into_py(py, async move {
                loop {
                    thread::sleep(Duration::from_secs(60));
                    let mut rpm = current_rpm.lock().unwrap();
                    *rpm = 0;
                }
            }));
        }
    }

    fn check_or_wait(&self, py: Python) -> PyResult<bool> {
        if let Some(max_rpm) = self.max_rpm {
            let mut rpm = self.current_rpm.lock().unwrap();
            if *rpm < max_rpm {
                *rpm += 1;
                Ok(true)
            } else {
                self.logger.call_method1(py, "log", ("info", "Max RPM reached, waiting for next minute to start."))?;
                thread::sleep(Duration::from_secs(60));
                *rpm = 1;
                Ok(true)
            }
        } else {
            Ok(true)
        }
    }

    fn stop_rpm_counter(&mut self, py: Python) {
        if let Some(timer) = &self.timer {
            timer.call_method0(py, "cancel").unwrap();
            self.timer = None;
        }
    }
}

#[pymodule]
fn rpm_controller(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RPMController>()?;
    Ok(())
}
