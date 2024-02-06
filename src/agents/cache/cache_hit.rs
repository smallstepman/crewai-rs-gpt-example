use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CacheHit<T> {
    action: T,
    cache: CacheHandler,
}

#[derive(Debug, Deserialize, Serialize)]
struct CacheHandler {
    // Implement the fields of CacheHandler struct
    // according to your requirements
}

// Implement the methods and functions for CacheHit and CacheHandler structs
// based on your use case and requirements
