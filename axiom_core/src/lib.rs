//! Axiom Core
//!
//! Core primitives, types, and foundational abstractions for the entire Nexus/Axiom stack.

#![warn(missing_docs)]

pub mod prelude;

/// Returns the current version of axiom-core.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }
}