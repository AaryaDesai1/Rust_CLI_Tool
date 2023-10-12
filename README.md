# aad64_command_line
Package a Rust Script into a Command-Line Tool
[![Clippy](https://github.com/nogibjj/aad64_command_line/actions/workflows/actions.yml/badge.svg)](https://github.com/nogibjj/aad64_command_line/actions/workflows/actions.yml)

## Summary:
This week's mini project was an introduction to coding in Rust (specifically a Rust CLI project). For the same, I created a project that shuffled through cards and retrieved the number of cards the user specified in the command line. Below are more details of the same.

### Contents:
```
aad64_command_line/
├── .github/
│   └── workflows/actions.yml
├── .gitignore
├── src/
│   ├── lib.rs
│   └── main.rs
├── Cargo.toml
├── Makefile
└── README.md
```

### 1. [actions.yml](https://github.com/nogibjj/aad64_command_line/actions/workflows/actions.yml)
This is a GitHub Actions workflow file named `Clippy`:
- **Name**: Clippy
- **Triggers**: It is triggered on both push events to the `main` branch and pull requests.
- **Jobs**:
  - **build**:
    - **Environment**: It runs on an Ubuntu environment.
    - **Steps**:
      - `actions/checkout@v1`: Checks out the repository.
      - `actions-rs/toolchain@v1`: Sets up the Rust toolchain with Clippy and Rustfmt.
      - **Format**: Executes the `make format` command.
      - **Lint**: Executes the `make lint` command.
      - **Test**: Executes the `make test` command.
This workflow automates the process of formatting, linting, and testing a Rust project using GitHub Actions whenever there is a push to the `main` branch or a pull request is opened.
As seen in the badge above, the project is passing the entire CI/CD build without any errors. Below is also a screenshot of the project passing the test function written in `lib.rs`.
<p align = 'center'><img width="837" alt="Screenshot 2023-10-11 at 4 26 29 PM" src="https://github.com/nogibjj/aad64_command_line/assets/143753050/5317cfaf-9df7-4e1d-884c-43d9f0a55e21"></p>

### 2. src Folder:
1. __[lib.rs](https://github.com/nogibjj/aad64_command_line/src/lib.rs)__: Has two functions, one which specifies the suits a card can have (`create_suit`) and the other that specifies it's values (`create_value`).
2. __[main.rs](https://github.com/nogibjj/aad64_command_line/src/main.rs)__: This file calls the functions written in lib.rs. It then asks the user for a command line input, which here, is the number of cards the user wants to return from a shuffled deck. The main function then goes on to shuffle the suits and values and then sip them together to create the user specified number of cards and returns them in the command line. The image below shows you an example of the same:

<img width="1000" alt="Screenshot 2023-10-11 at 4 29 11 PM" src="https://github.com/nogibjj/aad64_command_line/assets/143753050/95da4673-9f98-4b97-9a41-c07ae93f7b10">


### 3. [Cargo.toml](https://github.com/nogibjj/aad64_command_line/Cargo.toml)
This file is the `Cargo.toml` manifest for a Rust project named `cli-customize-cards` with dependencies on `clap`, `csv`, and `rand` libraries. It specifies the project version, edition, and library configuration.

### 4. [Makefile](https://github.com/nogibjj/aad64_command_line/Makefile)
This is a `Makefile` used for automating various tasks in a Rust project:
- `format`: Invokes `cargo fmt` to automatically format the code according to Rust style guidelines.
- `lint`: Executes `cargo clippy` to perform linting and static analysis to catch potential issues or non-idiomatic code.
- `test`: Runs the project's test suite using `cargo test`.
- `run`: Launches the project with `cargo run`.
- `all`: Combines the tasks `format`, `lint`, `test`, and `run` to perform all common project tasks in sequence.

