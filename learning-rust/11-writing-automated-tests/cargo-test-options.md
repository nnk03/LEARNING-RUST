```sh
cargo test -- --test-threads = 1 # for removing parallelism
cargo test -- --show-output
cargo test test_name # run the test functions that has same name with test_name
cargo test -- --ignored
cargo test -- --include-ignored

# for integration tests
cargo test --test file_name # run the integration test file with the name file_name.rs
```
