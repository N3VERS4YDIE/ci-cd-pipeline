# 🦀 Rust CI/CD Pipeline

An intuitive example of CI/CD implementation using GitHub Actions with Rust.

##  Overview

This project demonstrates a comprehensive CI/CD pipeline for Rust applications using GitHub Actions. It includes:

- Automated testing across multiple platforms
- Code formatting checks with `rustfmt`
- Linting with `clippy`
- Security auditing
- Code coverage reporting
- Automated releases with pre-built binaries
- Binary checksums for verification

## Project Structure

```bash
ci-cd-pipeline/
├── .github/
│   └── workflows/
│       ├── ci.yml          # Continuous Integration pipeline
│       └── cd.yml          # Continuous Deployment pipeline
├── src/
│   └── main.rs             # Application code with tests
├── Cargo.toml              # Project configuration
└── README.md               # This file
```

## CI Pipeline (Continuous Integration)

The CI pipeline runs on every push and pull request to `main` and `develop` branches.

### Pipeline Stages

1. **Format Check**
   - Ensures code follows Rust formatting standards
   - Uses `cargo fmt`

2. **Clippy Linting**
   - Catches common mistakes and improves code quality
   - Treats warnings as errors for strictness

3. **Test Suite**
   - Runs on multiple platforms: Linux, macOS, Windows
   - Tests against stable and beta Rust versions
   - Includes both debug and release mode tests

4. **Code Coverage**
   - Generates coverage reports using `cargo-tarpaulin`
   - Uploads results to Codecov

5. **Security Audit**
   - Checks dependencies for known security vulnerabilities
   - Uses `cargo-audit`

6. **Build Binary**
   - Creates release binaries for multiple platforms
   - Uploads artifacts for download

## CD Pipeline (Continuous Deployment)

The CD pipeline triggers when you push a version tag (e.g., `v1.0.0`).

### Release Process

1. **Create GitHub Release**
   - Automatically creates a release from the tag

2. **Build Cross-Platform Binaries**
   - Linux (x86_64)
   - macOS (x86_64 and ARM64)
   - Windows (x86_64)

3. **Generate Checksums**
   - SHA-256 checksums for all binaries

4. **Upload Assets**
   - Compressed binaries and checksums attached to release

5. **Publish to crates.io** (Optional)
   - Publishes the crate if configured

## Setup Instructions

### 1. Clone and Test Locally

```bash
# Clone the repository
git clone https://github.com/N3VER4YDIE/ci-cd-pipeline.git
cd ci-cd-pipeline

# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Build release
cargo build --release
```

### 2. Configure GitHub Repository

#### Required Secrets (for CD pipeline)

If you want to publish to crates.io, add this secret:

1. Go to your repository settings
2. Navigate to **Secrets and variables** → **Actions**
3. Add `CARGO_TOKEN` with your crates.io API token

#### Enable GitHub Actions

1. Go to the **Actions** tab in your repository
2. Enable workflows if prompted

### 3. Trigger CI

CI runs automatically on:

- Push to `main` or `develop` branches
- Pull requests to `main` or `develop` branches

### 4. Create a Release

To trigger the CD pipeline and create a release:

```bash
# Create and push a version tag
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

This will:

- Run all CI checks
- Build binaries for all platforms
- Create a GitHub release
- Upload artifacts

## Application Details

The application is a simple calculator demonstrating:

- Basic arithmetic operations (add, subtract, multiply, divide)
- Error handling (division by zero)
- Comprehensive unit tests
- Documentation

### Run the Application

```bash
cargo run
```

Expected output:

```txt
Calculator Operations:
10 + 5 = 15
10 - 5 = 5
10 * 5 = 50
10 / 5 = 2

✅ All operations completed successfully!
```

## Testing

Run all tests:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

Generate coverage report (requires `cargo-tarpaulin`):

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Development Tools

### Install Required Tools

```bash
# Rust formatter
rustup component add rustfmt

# Rust linter
rustup component add clippy

# Security audit
cargo install cargo-audit

# Coverage tool
cargo install cargo-tarpaulin
```

### Pre-commit Checks

Run these before committing:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release
```

## Best Practices Demonstrated

1. **Comprehensive Testing**: Multiple platforms and Rust versions
2. **Code Quality**: Formatting and linting enforcement
3. **Security**: Automated vulnerability scanning
4. **Caching**: Efficient builds with dependency caching
5. **Cross-Platform**: Builds for Linux, macOS, and Windows
6. **Documentation**: Clear README and inline comments
7. **Versioning**: Semantic versioning with Git tags
8. **Artifacts**: Pre-built binaries for easy distribution

## Learn More

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust CI/CD Best Practices](https://www.lpalmieri.com/posts/fast-rust-docker-builds/)

## License

This project is provided as an educational example. Feel free to use it as a template for your own projects!
