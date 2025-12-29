<!--
SPDX-FileCopyrightText: 2025 {{author_name}} <{{author_email}}>

SPDX-License-Identifier: MIT
-->

<p align="center">
  <h1 align="center">{{project-name}}</h1>
  <p align="center">
    <strong>{{description}}</strong>
  </p>
</p>

<p align="center">
  <a href="https://crates.io/crates/{{project-name}}">
    <img src="https://img.shields.io/crates/v/{{project-name}}.svg?style=for-the-badge" alt="Crates.io"/>
  </a>
  <a href="https://docs.rs/{{project-name}}">
    <img src="https://img.shields.io/docsrs/{{project-name}}?style=for-the-badge" alt="Documentation"/>
  </a>
</p>

<p align="center">
  <a href="https://github.com/{{gh_username}}/{{project-name}}/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/{{gh_username}}/{{project-name}}/ci.yml?style=for-the-badge" alt="CI Status"/>
  </a>
  <a href="https://codecov.io/gh/{{gh_username}}/{{project-name}}">
    <img src="https://img.shields.io/codecov/c/github/{{gh_username}}/{{project-name}}?style=for-the-badge" alt="Coverage"/>
  </a>
</p>

<p align="center">
  <a href="https://github.com/{{gh_username}}/{{project-name}}/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge" alt="License: MIT"/>
  </a>
  <a href="https://api.reuse.software/info/github.com/{{gh_username}}/{{project-name}}">
    <img src="https://img.shields.io/reuse/compliance/github.com%2F{{gh_username}}%2F{{project-name}}?style=for-the-badge" alt="REUSE Compliant"/>
  </a>
</p>

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
{{project-name}} = "0.1"
```

## Quick Start

```rust
use {{crate_name}}::greet;

fn main() {
    println!("{}", greet("World"));
}
```

## Features

- Feature 1
- Feature 2
- Feature 3

## Documentation

- [API Documentation](https://docs.rs/{{project-name}})
- [Examples](https://github.com/{{gh_username}}/{{project-name}}/tree/main/examples)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.
