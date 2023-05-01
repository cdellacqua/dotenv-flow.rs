[![Crates.io](./badges/crates.io.svg)](https://crates.io/crates/dotenv-flow)


# dotenv-flow.rs

A fork of [dotenv-rs](https://github.com/dotenv-rs/dotenv) that adds support for the popular dotenv-flow
loading strategy.


The dotenv-flow strategy works as follows:

- load .env.local
- if a DOTENV_ENV environment variable is set, load .env.{DOTENV_ENV}.local (e.g. .env.staging.local)
- if a DOTENV_ENV environment variable is set, load .env.{DOTENV_ENV} (e.g. .env.staging)
- load .env

Each step will only load variables that are not already present in the environment, so for example variables
in the .env.local file will have the highest priority, followed .env.{DOTENV_ENV}.local and so on.

## HowTo

### Installation

```sh
cargo add dotenv-flow
```

### Usage

To use this package, add the following line to your main function to load the environment variables from available `.env.*` files:

```rs
fn main() {
  dotenv_flow::dotenv_flow().ok();
}
```


## Tests

To test this project, make sure you pass `--test-threads=1` to `cargo test`, e.g.

```sh
cargo test -- --test-threads=1
```

This is necessary because `cargo test` runs tests in multiple threads by default, but environment variables are process-globals, therefore we need to limit concurrency to avoid race conditions.
