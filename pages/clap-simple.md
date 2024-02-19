---
title: Clap - getting started accepting command line parameters in Rust
timestamp: 2023-12-23T12:00:01
author: szabgab
published: true
description: The simplest example using Clap to accept command line parameters in Rust.
tags:
    - Clap
    - cli
    - derive
    - struct
    - arg
    - long
    - help
---

A minimal example of using [clap](/clap) to handle command line parameters.

## Create crate

```
cargo new simple-example
cd simple example
```


## Add clap as a dependency

```
cargo add clap --features derive
```

As of this writing clap has two interfaces. The higher-level one and the recommended one is the derive interface.
In order to use it we need to add the `derive` feature.

The resulting `Cargo.toml` file will look like this, though by the time you read this the version number will be probably higher.

{% include file="examples/clap/simple-example/Cargo.toml" %}


## The Code

{% include file="examples/clap/simple-example/src/main.rs" %}

## Usage

If we just run the program in will complain of a missing command line parameter called `--host`, but also offer us the `--help` flag.

```
$ cargo run

error: the following required arguments were not provided:
  --host <HOST>

Usage: simple-example --host <HOST>

For more information, try '--help'.
```

## Help

```
cargo run --help
```

will give us help on `cargo` itself. Not what we wanted.

```
cargo run -- --help
```

will work fine and show us this help:

```
Usage: simple-example --host <HOST>

Options:
      --host <HOST>
  -h, --help         Print help
```

The extra `--` is needed to separate the flags of `cargo` from those of your application.


## Help in the compiled binary

However, don't worry. If you take the compiled binary (either the debug one or the released one later on), then the user does not
need to provide the extra `--`:

Try this, for example:

```
./target/debug/simple-example  --help
```

## Providing more detailed help message

In another article you can see how to [provide additional text for the help message of each parameter](/clap-help-text) separately by adding the `help` argument.

## Passing the required parameter

```
cargo run -- --host 127.0.0.1
```

or if we would like to use the compiled binary directly then the `--` is not needed:

```
./target/debug/simple-example  --host 127.0.0.1
```


## Explanation

We create a `struct` that will describe the command line parameters.

The name of the `struct` does not matter, but I like to call it `Cli`.

What is important is that it has the `#[derive(Parser)]` on it to
add the [cli::Parser trait](https://docs.rs/clap/latest/clap/trait.Parser.html).

The `struct` has one attribute called `host` having type `String` and we add the `long` argument so `clap` will expected a named command line flag.

```rust
    #[arg(long)]
    host: String,
```

Then we call the [parse](https://docs.rs/clap/latest/clap/trait.Parser.html#method.parse) method that will return us an instance of the `Cli` struct.

```rust
    let args = Cli::parse();
```

Once we have this we can access the `args.host` attribute of the struct for which the value was received from the command line.



## Usage and Help


Clap automatically added the Usage message and the `--help` flag.


