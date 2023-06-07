# rgit

![REUSE Compliance](https://github.com/henri-egger/rgit/actions/workflows/reuse.yml/badge.svg)
[![REUSE status](https://api.reuse.software/badge/github.com/henri-egger/rgit)](https://api.reuse.software/info/github.com/henri-egger/rgit)

This is a small implementation of git in the Rust programming language, i made it to learn Rust and better understand git at the same time.

## Features

rgit is more of an experiment and therefore lacking many of gits features. Also, it currently only works on unix systems.

Implemented features:

-   Initializing a repository (`init`)
-   Adding to staging area (`add`)
-   Checking status (`status`)
-   Committing (`commit`)
-   Logging commits (`log`)
-   Loading a previous commit (`checkout`)
-   Some commands useful for development (list them with `dev -h`)

## Usage

rgit uses Cargo as its build tool and package manager. To run rgit yourself, install Cargo on your machine.

Either compile rgit to an executable and run it:

```bash
cargo build
```

Or use the builtin compile and run command:

```bash
cargo run
```

Get a list of all available commands (implemented or not) by running the `help` command.
