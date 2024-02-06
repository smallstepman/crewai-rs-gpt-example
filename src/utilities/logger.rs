use std::collections::HashMap;

struct Logger {
    verbose_level: u32,
}

impl Logger {
    fn new(verbose_level: u32) -> Self {
        let verbose_level = if verbose_level != 0 { verbose_level } else { 2 };
        Logger { verbose_level }
    }

    fn log(&self, level: &str, message: &str) {
        let level_map: HashMap<&str, u32> = [("debug", 1), ("info", 2)].iter().cloned().collect();
        if self.verbose_level > 0 && level_map.get(level).unwrap_or(&0) <= &self.verbose_level {
            println!("[{}]: {}", level.to_uppercase(), message);
        }
    }
}

fn main() {
    let logger = Logger::new(0);
    logger.log("debug", "Debug message");
}
