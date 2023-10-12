# aad64_command_line
Package a Rust Script into a Command-Line Tool
[![Clippy](https://github.com/nogibjj/aad64_command_line/actions/workflows/actions.yml/badge.svg)](https://github.com/nogibjj/aad64_command_line/actions/workflows/actions.yml)

## Summary:
This week's mini project was an introduction to coding in Rust (specifically a Rust CLI project). For the same, I created a project that shuffled through cards and retrieved the number of cards the user specified in the command line. Below are more details of the same.

### Contents:

aad64_command_line/
├── .github/
│   └── workflows/[actions.yml](https://github.com/nogibjj/aad64_command_line/actions/workflows/actions.yml)
├── .gitignore
├── src/
│   ├── [lib.rs](https://github.com/nogibjj/aad64_command_line/src/lib.rs)
│   └── [main.rs](https://github.com/nogibjj/aad64_command_line/src/main.rs)
├── [Cargo.toml](https://github.com/nogibjj/aad64_command_line/Cargo.toml)
├── [Makefile](https://github.com/nogibjj/aad64_command_line/Makefile)
└── README.md


### src Folder:
1. lib.rs: Has two functions, one which specifies the suits a card can have (`create_suit`) and the other that specifies it's values (`create_value`).
2. main.rs: This file calls the functions written in lib.rs. It then asks the user for a command line input, which here, is the number of cards the user wants to return from a shuffled deck. The main function then goes on to shuffle the suits and values and then sip them together to create the user specified number of cards and returns them in the command line. The image below shows you an example of the same:

<img width="1000" alt="Screenshot 2023-10-11 at 4 29 11 PM" src="https://github.com/nogibjj/aad64_command_line/assets/143753050/95da4673-9f98-4b97-9a41-c07ae93f7b10">


