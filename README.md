# rustamodb
[![Build Status](https://travis-ci.org/cmac4603/rustamodb.svg?branch=master)](https://travis-ci.org/cmac4603/rustamodb)
[![Crates.io](https://img.shields.io/crates/v/rustamodb.svg)](https://crates.io/crates/rustamo)

_Currently **very** early stages WIP._

`rustamodb` is inspired by [pynamodb](https://pynamodb.readthedocs.io/en/latest/) and is a simple to use library for AWS DynamoDB.

## Getting Started
### To use `rustamodb` in your rust codebade
In your projects `Cargo.toml` file (find the latest version on [crates.io](https://crates.io/crates/rustamodb)):
```toml
[dependencies]
rustamodb = "0.0.6"
```

In `main.rs`:
```rust
extern crate rustamodb;

fn main() {
    // scan a dynamodb table
    match rustamodb::scan(&"my_dynamodb_table") {
        Ok(scan_output) => {
            for item in scan_output {
                println!("{:?}", item);
            }
        },
        Err(_) => (),
    }
}
```
_More documentation to come... see the [docs.rs](https://docs.rs/rustamodb/) page_

## Running the tests
### Cargo tests
- Runs unit tests in code
- Tests code found in documentation lines in code

```bash
cargo test
```
### Coding style tests
Currently not implemented
<!-- Would like to implement clippy at some point -->

## Built With
* [rusoto](https://rusoto.org/) - AWS SDK for Rust

## Contributing
### Your First Contribution
- review a [Pull Request](https://github.com/cmac4603/rustamodb/pulls)
- fix an [Issue](https://github.com/cmac4603/rustamodb/issues)
- update the documentation

## Contributors
See the list of [contributors](https://github.com/cmac4603/rustamodb/contributors) who participated in this project.

## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments
* [rusoto](https://rusoto.org/)
* [pynamodb](https://pynamodb.readthedocs.io/en/latest/)
