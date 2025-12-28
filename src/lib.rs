// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! # {{crate_name}}
//!
//! A short description of what this crate does.
//!
//! ## Example
//!
//! ```rust
//! use {{crate_name}}::hello;
//!
//! let message = hello();
//! assert_eq!(message, "Hello, world!");
//! ```

/// Returns a greeting message.
///
/// # Example
///
/// ```rust
/// use {{crate_name}}::hello;
///
/// assert_eq!(hello(), "Hello, world!");
/// ```
#[must_use]
pub fn hello() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
