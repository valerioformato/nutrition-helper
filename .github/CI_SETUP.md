# GitHub Actions CI/CD Setup

## Overview

This project uses GitHub Actions for continuous integration and continuous deployment. The workflows automatically test, lint, and build the application on every push and pull request.

## Workflows

### 1. CI Workflow (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop` branches

**Jobs:**

#### `test-rust`
Tests the Rust backend across multiple operating systems.

- **Matrix Strategy:** Runs on Ubuntu, Windows, and macOS
- **Steps:**
  1. Checkout code
  2. Install Rust toolchain with rustfmt and clippy
  3. Cache Rust dependencies (Swatinem/rust-cache)
  4. Check code formatting with `cargo fmt`
  5. Run linter with `cargo clippy` (fails on warnings)
  6. Run all library tests with `cargo test --lib`
  7. Build the project with `cargo build`

#### `test-frontend`
Tests the frontend TypeScript code.

- **Platform:** Ubuntu only (for speed)
- **Steps:**
  1. Checkout code
  2. Setup Node.js 20 with npm cache
  3. Install dependencies with `npm ci`
  4. Check TypeScript types with `npm run check`
  5. Run linting with `npm run lint` (non-blocking)

#### `build-tauri`
Full integration test - builds complete Tauri application.

- **Platform:** Ubuntu only
- **Depends on:** `test-rust` and `test-frontend` must pass first
- **Steps:**
  1. Install system dependencies (webkit, gtk, etc.)
  2. Build complete Tauri application
  3. Verifies that frontend and backend integrate correctly

**Estimated Runtime:** 15-25 minutes total

### 2. Coverage Workflow (`.github/workflows/coverage.yml`)

**Triggers:**
- Push to `main` branch
- Pull requests to `main` branch

**Job: `coverage`**

- **Platform:** Ubuntu only
- **Steps:**
  1. Install cargo-tarpaulin for coverage analysis
  2. Generate coverage report in XML format
  3. Upload to Codecov (requires `CODECOV_TOKEN` secret)
  4. Check coverage threshold (fails if < 85%)

**Coverage Targets:**
- Backend: 85% minimum (enforced)
- Frontend: 70% target (Phase 7)

**Estimated Runtime:** 5-10 minutes

## Setup Instructions

### Required GitHub Secrets

For Codecov integration, add the following secret to your repository:

1. Go to repository Settings → Secrets and variables → Actions
2. Add new repository secret:
   - **Name:** `CODECOV_TOKEN`
   - **Value:** Get from https://codecov.io after linking your repository

### Local Testing

Before pushing, test locally:

```bash
# Run Rust tests
cd src-tauri
cargo test --lib

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Check TypeScript
npm run check

# Run linting
npm run lint
```

### Coverage Reports Locally

```bash
# Install tarpaulin (one-time)
cargo install cargo-tarpaulin

# Generate coverage report
cd src-tauri
cargo tarpaulin --lib --out Html --output-dir ../coverage

# Open coverage/index.html in browser
```

## Badge Status

Add these badges to your README:

```markdown
[![CI](https://github.com/valerioformato/nutrition-helper/actions/workflows/ci.yml/badge.svg)](https://github.com/valerioformato/nutrition-helper/actions/workflows/ci.yml)
[![Code Coverage](https://github.com/valerioformato/nutrition-helper/actions/workflows/coverage.yml/badge.svg)](https://github.com/valerioformato/nutrition-helper/actions/workflows/coverage.yml)
```

## Caching Strategy

The workflows use caching to speed up builds:

- **Rust Cache:** `Swatinem/rust-cache@v2` caches target directory and Cargo registry
- **npm Cache:** Built into `setup-node@v4` action
- **Cache Key:** Based on Cargo.lock and package-lock.json

Expected speedup: 2-3x faster on subsequent runs.

## Troubleshooting

### Clippy Warnings Fail CI

Clippy is configured to fail on warnings (`-D warnings`). Fix locally:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Coverage Below Threshold

If coverage drops below 85%:

1. Check which files/functions lack coverage
2. Add tests for uncovered code
3. Run `cargo tarpaulin --lib` to verify locally

### Build Fails on Specific OS

Matrix testing catches OS-specific issues. Debug locally with:

```bash
# On the failing OS
cargo build --verbose
cargo test --verbose
```

## Future Enhancements

- [ ] Add frontend test coverage with Istanbul/c8
- [ ] Set up automated releases with GitHub Actions
- [ ] Add performance benchmarking workflow
- [ ] Create deployment workflow for releases
- [ ] Add security scanning (Dependabot, cargo-audit)
- [ ] Set up automatic dependency updates

## Maintenance

- **Update Rust version:** Modify `.github/workflows/ci.yml` toolchain version
- **Update Node version:** Modify `node-version` in setup-node steps
- **Add new tests:** Automatically picked up by `cargo test`
- **Coverage threshold:** Modify percentage in `coverage.yml` line 40

---

**Last Updated:** November 5, 2025
