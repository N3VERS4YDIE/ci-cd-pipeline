# Quick Start Guide

This guide will help you get started with the CI/CD workflow.

## Prerequisites

- Rust installed (<https://rustup.rs/>)
- Git installed
- A GitHub account

## Local Development

### 1. Build and Run

```bash
# Build the project
cargo build

# Run the application
cargo run

# Build optimized release
cargo build --release
```

### 2. Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_add
```

### 3. Code Quality Checks

```bash
# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Run linter
cargo clippy

# Run linter (strict mode)
cargo clippy -- -D warnings
```

## Setting Up CI/CD

### 1. Push to GitHub

```bash
git init
git add .
git commit -m "Initial commit with CI/CD setup"
git remote add origin https://github.com/N3VER4YDIE/ci-cd-pipeline.git
git push -u origin main
```

### 2. Verify CI Pipeline

1. Go to your GitHub repository
2. Click on the "Actions" tab
3. You should see the CI workflow running
4. Wait for it to complete (green checkmark = success)

### 3. Create Your First Release

```bash
git tag -a v0.1.0 -m "First release"
git push origin v0.1.0
```

This will trigger the CD pipeline which will:

- Build binaries for Linux, macOS, and Windows
- Create a GitHub release
- Upload the binaries as release assets

### 4. Download Release Binaries

After the CD pipeline completes:

1. Go to the "Releases" section in your GitHub repository
2. Find your release (v0.1.0)
3. Download the appropriate binary for your platform
4. Extract and run it

## Optional: Codecov Integration

To enable code coverage reporting:

1. Go to [codecov.io](https://codecov.io/)
2. Sign in with GitHub
3. Enable your repository
4. The CI pipeline will automatically upload coverage reports

## Optional: Publish to crates.io

To publish your crate:

1. Get an API token from [crates.io](https://crates.io/settings/tokens)
2. Add it as a secret named `CARGO_TOKEN` in your GitHub repository
3. The CD pipeline will automatically publish when you create a release

## Troubleshooting

### CI Fails on Format Check

```bash
# Fix formatting
cargo fmt
git add .
git commit -m "Fix formatting"
git push
```

### CI Fails on Clippy

```bash
# See what's wrong
cargo clippy

# Fix the issues, then
git add .
git commit -m "Fix clippy warnings"
git push
```

### Tests Fail

```bash
# Run tests locally to see the error
cargo test

# Fix the code, then
git add .
git commit -m "Fix failing tests"
git push
```

## Next Steps

- Customize the calculator with more operations
- Add more comprehensive tests
- Experiment with different CI/CD configurations
- Try deploying to a container registry (Docker)
- Add integration tests

Happy coding! 🦀
