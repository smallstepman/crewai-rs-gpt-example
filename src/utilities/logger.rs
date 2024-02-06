struct Logger {
    verbose_level: u8,
}

impl Logger {
    fn new(verbose_level: u8) -> Logger {
        Logger { verbose_level }
    }

    fn log(&self, level: &str, message: &str) {
        let level_map = [("debug", 1), ("info", 2)].iter().cloned().collect::<std::collections::HashMap<&str, u8>>();
        if self.verbose_level > 0 {
            if let Some(&level_value) = level_map.get(level) {
                if level_value <= self.verbose_level {
                    println!("[{}]: {}", level.to_uppercase(), message);
                }
            }
        }
    }
}
