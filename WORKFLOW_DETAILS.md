# CI/CD Workflow Configuration Details

## Overview

This document explains the GitHub Actions workflows in detail.

## CI Workflow (`.github/workflows/ci.yml`)

Runs on: Push and PR to `main` and `develop` branches

### Jobs

#### 1. Format Check

- **Purpose**: Ensure consistent code formatting
- **Tool**: `rustfmt`
- **Command**: `cargo fmt -- --check`
- **Fails if**: Code is not properly formatted

#### 2. Clippy Linting

- **Purpose**: Catch common mistakes and improve code quality
- **Tool**: `clippy`
- **Command**: `cargo clippy -- -D warnings`
- **Fails if**: Any warnings are detected
- **Caching**: Uses GitHub Actions cache for faster builds

#### 3. Test Suite

- **Purpose**: Verify code correctness across platforms
- **Platforms**: Ubuntu, macOS, Windows
- **Rust Versions**: Stable, Beta
- **Test Modes**: Debug and Release
- **Total Combinations**: 6 (3 platforms × 2 versions)
- **Caching**: Cargo registry, git index, and target directory

#### 4. Code Coverage

- **Purpose**: Track test coverage
- **Tool**: `cargo-tarpaulin`
- **Output**: Codecov integration
- **Runs on**: Ubuntu only (fastest)
- **Timeout**: 120 seconds

#### 5. Security Audit

- **Purpose**: Detect known vulnerabilities in dependencies
- **Tool**: `cargo-audit`
- **Database**: RustSec Advisory Database
- **Fails if**: Any vulnerabilities found

#### 6. Build Binary

- **Purpose**: Verify release builds work
- **Depends on**: Format, Clippy, and Test jobs
- **Platforms**: Linux, macOS, Windows
- **Uploads**: Artifacts for each platform

### Caching Strategy

The workflow uses three types of caches:

1. **Cargo Registry**: `~/.cargo/registry`
2. **Cargo Git Index**: `~/.cargo/git`
3. **Build Artifacts**: `target/`

Cache key includes `Cargo.lock` hash for invalidation on dependency changes.

## CD Workflow (`.github/workflows/cd.yml`)

Runs on: Push of version tags (e.g., `v1.0.0`)

### Jobs

#### 1. Create Release

- **Purpose**: Create GitHub release from tag
- **Output**: Upload URL for assets
- **Includes**: Release notes template

#### 2. Build & Release

- **Purpose**: Build and upload platform-specific binaries
- **Targets**:
  - `x86_64-unknown-linux-gnu` (Linux)
  - `x86_64-apple-darwin` (macOS Intel)
  - `aarch64-apple-darwin` (macOS ARM)
  - `x86_64-pc-windows-msvc` (Windows)

**Process for each target**:

1. Build release binary
2. Strip debug symbols (Linux/macOS)
3. Create compressed archive (.tar.gz or .zip)
4. Generate SHA-256 checksum
5. Upload to GitHub Release

#### 3. Publish to Crates.io (Optional)

- **Purpose**: Publish package to Rust package registry
- **Requires**: `CARGO_TOKEN` secret
- **Continues on error**: Won't fail if already published

## Environment Variables

### CI Workflow

- `CARGO_TERM_COLOR: always` - Colored output in logs

### CD Workflow

- `CARGO_TERM_COLOR: always` - Colored output in logs
- `GITHUB_TOKEN` - Automatically provided by GitHub Actions

## Required Secrets

### For CD Pipeline

- `CARGO_TOKEN` (Optional): Token from crates.io for publishing

**To add secrets**:

1. Go to repository Settings
2. Navigate to Secrets and variables → Actions
3. Click "New repository secret"
4. Add name and value

## Workflow Triggers

### CI (`ci.yml`)

```yaml
on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]
```

### CD (`cd.yml`)

```yaml
on:
  push:
    tags:
      - 'v*.*.*'
```

## Customization Guide

### Add New Platforms

To add a new build target in CD workflow:

```yaml
- target: aarch64-unknown-linux-gnu
  os: ubuntu-latest
  name: ci-cd-pipeline-linux-arm64
```

### Change Test Matrix

To modify tested platforms/versions in CI:

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable, beta, nightly]  # Added nightly
```

### Add Code Quality Tools

Example: Add `cargo-deny` for dependency checking:

```yaml
- name: Install cargo-deny
  run: cargo install cargo-deny

- name: Check dependencies
  run: cargo deny check
```

### Modify Caching

To add more cache paths:

```yaml
- name: Cache additional directory
  uses: actions/cache@v3
  with:
    path: some/directory
    key: ${{ runner.os }}-custom-${{ hashFiles('**/Cargo.lock') }}
```

## Best Practices

1. **Always cache dependencies** to reduce build time
2. **Use matrix builds** for cross-platform testing
3. **Separate concerns** (formatting, linting, testing)
4. **Fail fast** on critical checks (format, clippy)
5. **Upload artifacts** for debugging and distribution
6. **Version your actions** for reproducibility
7. **Use semantic versioning** for releases

## Troubleshooting

### Slow CI Builds

- Ensure caching is working
- Check cache hit rates in workflow logs
- Consider reducing test matrix

### Failed Releases

- Verify tag format matches `v*.*.*`
- Check `CARGO_TOKEN` secret is set
- Ensure `Cargo.toml` version matches tag

### Platform-Specific Failures

- Check platform-specific dependencies
- Review conditional compilation flags
- Test locally with target platform

## Performance Optimization

### Current Optimizations

1. Cargo caching (registry, git, target)
2. Parallel job execution
3. Conditional job dependencies
4. Artifact retention (90 days default)

### Further Optimizations

1. Use `sccache` for distributed caching
2. Split tests into parallel jobs
3. Use self-hosted runners for frequent builds
4. Pre-build Docker images with dependencies

## Monitoring

### Key Metrics to Track

- Build duration
- Cache hit rate
- Test pass rate
- Coverage percentage
- Release success rate

### GitHub Insights

- Go to Actions → Select workflow
- View run history and trends
- Check average run time

## Related Documentation

- [GitHub Actions Docs](https://docs.github.com/en/actions)
- [actions-rs Toolchain](https://github.com/actions-rs/toolchain)
- [Rust Cargo Book](https://doc.rust-lang.org/cargo/)
- [Semantic Versioning](https://semver.org/)

---

**Note**: Keep workflows updated with latest action versions for security and features.
