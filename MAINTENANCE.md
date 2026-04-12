# Maintenance

## Running Tests
```bash
cargo test
```

## Code Style
```bash
cargo fmt
cargo clippy
```

## Extending
- Add new `TraceType` variants in `src/trace.rs`
- Add new query methods to `SharedEnvironment` in `src/environment.rs`
- Update tests to maintain coverage above 15 test cases
