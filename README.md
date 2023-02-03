[![Rust CI/CD Pipeline](https://github.com/FengyiQuan/rust-mlops-template/actions/workflows/rust.yml/badge.svg)](https://github.com/FengyiQuan/rust-mlops-template/actions/workflows/rust.yml)

# rust-mlops-template

A work in progress to build out solutions in Rust for MLOPs.
This repo is more of a cookbook style. For a more gentle step by step guide to MLOps with Rust, please see my [lecture notes as a Rust MDBook here](https://nogibjj.github.io/rust-tutorial/).

![8-3-modern-rust-development](https://user-images.githubusercontent.com/58792/211929407-633fddb8-8f44-4d30-861c-30aaa7b481e3.png)

## Demo Hitlist (Will Solve hopefully almost every day/weekly)

- Do an [inline python example](https://github.com/fusion-engineering/inline-python)
- Train a model in PyTorch with CPU: https://github.com/LaurentMazare/tch-rs
- Train a model in PyTorch with GPU: https://github.com/LaurentMazare/tch-rs
- Serve out ONNX with a Rust web framework like Actix
- ONNX Command-Line Tool
- Simple async network example: (network discovery or chat system)
- Rust SQLite Example
- Rust AWS Lambda
- Simple Rust GUI
- Rust Whisper Tool with [C++ Bindings](https://github.com/tazz4843/whisper-rs)
- Fast Keyword Extraction (NLP)
- Emit Random Mediterranean Meals via CLI
- Web Assembly Rust

### Advanced Aspirational Demos

- Building a database in Rust
- Building a search engine in Rust
- Building a web server in Rust
- Building a batch processing systems in Rust
- Build a command-line chat system
- Build a locate clone
- Build a load-testing tool

## Motivation

One of the key goals of this project is to determine workflows that do not involve the #jcpennys (Jupyter, Conda, Pandas, Numpy, Sklearn) stack for #mlops. In particular I am not a fan of the conda installation tool (it is superfluous as [I demonstrate in the Python MLOps Template](https://github.com/nogibjj/mlops-template)) vs containerized workflows that use the Python Standard Library (Docker + pip + virtualenv) and this is a good excuse to find other solutions outside of that stack. For example:

- Why not also find a more performant Data Frame library, faster speed, etc.
- Why not have a compiler?
- Why not have a simple packaging solution?
- Why not have a very fast computational speed?
- Why not be able to write both for the Linux Kernel and general purpose scripting?
- Why not see if there is a better solution than Python (which is essentially two languages scientific python and regular Python)?
- Python is one of the least green languages in terms of energy efficiency, but [Rust is one of the best](https://greenlab.di.uminho.pt/wp-content/uploads/2017/10/sleFinal.pdf).

### In The Beginning Was the Command-Line

What could #mlops and #datascience look like in 2023 without #jupyternotebook and "God Tools" as the center of the universe? It could be the command line. In the beginning, it was the command line, and it may be the best solution for this domain.

"What would the engineer say after you had explained your problem and enumerated all the dissatisfactions in your life? He would probably tell you that life is a very hard and complicated thing; that no interface can change that; that anyone who believes otherwise is a sucker; and that if you don't like having choices made for you, you should start making your own." -Neal Stephensen

### Using Data (i.e. Data Science)

- StackOverflow https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages[states that #rust is on 7th year as the most loved language 87% of developers want to continue developing](https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages) in and ties with Python as the most wanted technology. Clearly there is traction.
- According to http://www.modulecounts.com/[Modulecounts] it looks like an exponential growth curve to Rust.
  ![Python vs Ruby vs Rust](https://user-images.githubusercontent.com/58792/209174014-cb3d7370-d8a2-4298-847b-f1e9f9f29a69.png)

## Getting Started

This repository is a GitHub Template and you can use it to create a new repository that uses [GitHub Codespaces](https://github.com/features/codespaces). It is pre-configured with [Rust](https://www.rust-lang.org/tools/install), [Cargo](https://crates.io/) and other useful extensions like [GitHub Copilot](https://github.com/features/copilot).

### Install and Setup

There are a few options:

- You can follow the [Official Install Guide for Rust](https://www.rust-lang.org/tools/install)
- Create a [repo with this template](https://github.com/nogibjj/rust-mlops-template)

Once you install you should check to see things work:

`rustc --version`

Other option is to run `make rust-version` which checks both the cargo and rust version.
To run everything locally do: `make all` and this will format/lint/test all projects in this repository.

### Rust CLI Tools Ecosystem

You can see there several tools which help you get things done in Rust:

```Makefile
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter
```

### Hello World Setup

This is an intentionally simple full end-to-end hello world example. I used some excellent ideas from @kyclark, author of the command-line-rust book from O'Reilly [here](https://github.com/kyclark/command-line-rust/tree/master/01_hello). You can recreate on your own following these steps

Create a project directory

- `cargo new hello`

This creates a structure you can see with `tree hello`

```bash
hello/
├── Cargo.toml
└── src
    └── main.rs
1 directory, 2 files
```

The `Cargo.toml` file is where the project is configured, i.e. if you needed to add a dependency.
The source code file has the following content in `main.rs`. It looks a lot like Python or any other modern language and this function prints a message.

```
fn main() {
    println!("Hello, world MLOPs!");
}
```

To run the project you cd into hello and run `cargo run` i.e. `cd hello && cargo run`. The output looks like the following:

```bash
@noahgift ➜ /workspaces/rust-mlops-template/hello (main ✗) $ cargo run
   Compiling hello v0.1.0 (/workspaces/rust-mlops-template/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/hello`
Hello, world MLOPs!
```

To run without all of the noise: `cargo run --quiet`.
To run the binary created `./target/debug/hello`

### Run with GitHub Actions

GitHub Actions uses a `Makefile` to simplify automation

```yaml
name: Rust CI/CD Pipeline
on:
  push:
    branches: ["main"]
  pull_request:
    branches: ["main"]
env:
  CARGO_TERM_COLOR: always
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v1
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          components: clippy, rustfmt
          override: true
      - name: update linux
        run: sudo apt update
      - name: update Rust
        run: make install
      - name: Check Rust versions
        run: make rust-version
      - name: Format
        run: make format
      - name: Lint
        run: make lint
      - name: Test
        run: make test
```

To run everything locally do: `make all`.

### [Simple Guessing Game](https://github.com/FengyiQuan/rust-mlops-template/tree/main/guessing-game)

Change into [`/guessing-game`] directory and run `cargo run` and you should see the following output. It allows user to guessing a number.

```bash
Guess the number!
Please input your guess.
4
You guessed: 4
Too small!
Please input your guess.
30
You guessed: 30
Too big!
Please input your guess.
15
You guessed: 15
You win!
```

### [Pattern Count CLI](https://github.com/FengyiQuan/rust-mlops-template/tree/main/pattern-count-cli)

Change into [`/pattern-count-cli`] directory and `run cargo run -- 1 'test.txt'` to list all lines that contains `1` in the file 'test.txt'.
Usage: `run cargo run -- <pattern> <file-path>`

### [Marco Polo AWS Lambda](https://github.com/FengyiQuan/rust-mlops-template/tree/main/marco-polo-lambda)

Deploy A singple function to AWS lambda.

Usage:
```bash
cargo lambda invoke --remote \
                --data-ascii '{"name": "Marco"}' \
                --output-format json \
                marco-polo-lambda
{
  "msg": "Marco says Polo",
  "req_id": "82c90ae1-4bce-4eac-8de2-c8952d2b376d"
}
```
