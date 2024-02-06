use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct RPMController {
    max_rpm: Option<i32>,
    current_rpm: i32,
    timer: Option<thread::JoinHandle<()>>,
    lock: Arc<Mutex<()>>,
}

impl RPMController {
    fn new(max_rpm: Option<i32>) -> Self {
        RPMController {
            max_rpm,
            current_rpm: 0,
            timer: None,
            lock: Arc::new(Mutex::new(())),
        }
    }

    fn reset_counter(&mut self) {
        if let Some(max_rpm) = self.max_rpm {
            self.current_rpm = 0;
            self.start_timer(max_rpm);
        }
    }

    fn check_or_wait(&mut self) -> bool {
        if let Some(max_rpm) = self.max_rpm {
            let lock = self.lock.clone();
            let mut guard = lock.lock().unwrap();
            if self.current_rpm < max_rpm {
                self.current_rpm += 1;
                true
            } else {
                println!("Max RPM reached, waiting for next minute to start.");
                self.wait_for_next_minute();
                self.current_rpm = 1;
                true
            }
        } else {
            true
        }
    }

    fn stop_rpm_counter(&mut self) {
        if let Some(timer) = self.timer.take() {
            timer.join().unwrap();
        }
    }

    fn wait_for_next_minute(&self) {
        thread::sleep(Duration::from_secs(60));
        let mut guard = self.lock.lock().unwrap();
        self.current_rpm = 0;
    }

    fn start_timer(&mut self, max_rpm: i32) {
        let lock = self.lock.clone();
        let timer = thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(60));
                let mut guard = lock.lock().unwrap();
                self.current_rpm = 0;
            }
        });
        self.timer = Some(timer);
    }
}

fn main() {
    let mut controller = RPMController::new(Some(10));
    controller.reset_counter();
    controller.check_or_wait();
    controller.stop_rpm_counter();
}
