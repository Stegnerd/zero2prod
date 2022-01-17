# Notes
## my notes as I go through this book

### Useful Commands
- cargo test ->run unit tests
- cargo fmt --all -- --check -> looks for unformatted code
- cargo clippy -- -D warnings -> runs lint and fails on warning
- cargo tarpaulin --ignore-tests -> runs code coverage - tests
- start db docker first time: ./scripts/db_init
- start it again when already running: SKIP_DOCKER=true ./scripts/init_db.sh

### Notes
- serde slower due to the fact that it is generic over the underlying data formats?
No, thanks to a process called monomorphization.
Every time a generic function is called with a concrete set of types, the Rust compiler will create
22You can look at serde_json’s serialize_seq implementation for confirmation: here. There is an optimisation for empty sequences (you immediately output []), but that is pretty much what is happening.
46
a copy of the function body replacing the generic type parameters with the concrete types. This allows the compiler to optimize each instance of the function body with respect to the concrete types involved: the result is no different from what we would have achieved writing down separate functions for each type, without using generics or traits. In other words, we do not pay any runtime costs for using generics23.
- seems gross to have db connection checked in but the env file is only for tests, not for after compiling like the yaml
