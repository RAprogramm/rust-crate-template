// SPDX-FileCopyrightText: 2025 {{author_name}} <{{author_email}}>
//
// SPDX-License-Identifier: MIT

//! # {{project-name}}
//!
//! {{description}}
//!
//! ## Quick Start
//!
//! ```rust
//! use {{crate_name}}::greet;
//!
//! let message = greet("World");
//! assert_eq!(message, "Hello, World!");
//! ```

/// Returns a greeting message.
///
/// # Arguments
///
/// * `name` - The name to greet
///
/// # Example
///
/// ```rust
/// use {{crate_name}}::greet;
///
/// assert_eq!(greet("Rust"), "Hello, Rust!");
/// ```
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn test_greet_empty() {
        assert_eq!(greet(""), "Hello, !");
    }
}
