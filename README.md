<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# Rust Crate Template

A production-ready template for Rust crates with CI/CD, coverage, and automated releases.

## Features

- Rust 2024 edition with MSRV 1.92
- GitHub Actions CI/CD pipeline
- Automated releases to crates.io
- Code coverage with Codecov
- Security auditing with cargo-deny
- REUSE-compliant licensing
- Dependabot auto-merge for minor/patch updates
- git-cliff for changelog generation

## Quick Start

1. Click "Use this template" on GitHub
2. Clone your new repository
3. Replace placeholders:

```bash
# Replace these in all files:
# {{crate_name}} -> your-crate-name
# {{github_username}} -> your-github-username

find . -type f \( -name "*.toml" -o -name "*.yml" -o -name "*.md" -o -name "*.rs" \) \
  -exec sed -i 's/{{crate_name}}/your-crate-name/g' {} \;

find . -type f \( -name "*.toml" -o -name "*.yml" -o -name "*.md" -o -name "*.rs" \) \
  -exec sed -i 's/{{github_username}}/your-github-username/g' {} \;
```

4. Update `Cargo.toml` with your details:
   - `authors`
   - `description`
   - `keywords`
   - `categories`

5. Set up secrets in GitHub repository settings:
   - `CARGO_REGISTRY_TOKEN` - for crates.io publishing
   - `CODECOV_TOKEN` - for coverage reports (optional)

## Project Structure

```
.
├── .github/
│   ├── workflows/
│   │   ├── ci.yml              # Main CI/CD pipeline
│   │   └── dependabot-automerge.yml
│   └── dependabot.yml
├── LICENSES/
│   └── MIT.txt
├── src/
│   └── lib.rs
├── Cargo.toml
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE -> LICENSES/MIT.txt
├── README.md
├── REUSE.toml
├── cliff.toml
├── codecov.yml
└── deny.toml
```

## CI Pipeline

| Stage | Jobs |
|-------|------|
| Checks | Format, Clippy, Docs, Security, REUSE |
| Tests | Unit tests, Doc tests, Coverage |
| Release | Changelog, Publish, GitHub Release |

## Development

```bash
# Format code
cargo +nightly fmt

# Lint
cargo clippy -- -D warnings

# Test
cargo test

# Coverage
cargo llvm-cov
```

## License

MIT License - see [LICENSE](LICENSE) for details.
