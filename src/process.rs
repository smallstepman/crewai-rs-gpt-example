use std::fmt;

#[derive(Debug)]
enum Process {
    Sequential,
    Hierarchical,
    // TODO: Consensual,
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Process::Sequential => write!(f, "sequential"),
            Process::Hierarchical => write!(f, "hierarchical"),
            // TODO: Process::Consensual => write!(f, "consensual"),
        }
    }
}
