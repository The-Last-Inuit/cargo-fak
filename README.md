<div>
    <img src="fak.png" alt="Cargo FAAAAAAK!"/>
</div>


# Cargo Faaaaaak!
- Are you tired of your past persona making decisions that make no sense?
- Are you often asking yourself: why did I make that decision? 

Fear no more, Cargo Faaaaaak! is an ADR tool for your Rust projects. 
You don't need help, you are human, you are fine. 
What you need is better tools, Cargo Faaaaaak! aims to help you out.

#### Usage

Add cargo-fak as a development dependency in your Cargo.toml () to use from crates.io:

```
[dev-dependencies]
cargo-fak="1.0.0"
```

You can get information about cargo-fak by typing:

```bash
$ cargo adr --help
Cargo Faaaaaak! is an ADR tool for your Rust projects. You don't need help, you are human, you are fine. What you need
is better tools, Cargo Faaaaaak! aims to help you out.

USAGE:
    cargo adr <TITLE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <TITLE>    A title of your ADR
```

To create a new record, just type:

```bash
$ cargo adr a-title-for-your-record
```

And you will find your new record at `docs/adr/TIMESTAMP_a-title-for-your-record.md`.
