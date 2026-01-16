<div>
    <img src="fak.png" alt="Cargo FAAAAAAK!"/>
</div>


# Cargo Faaaaaak!

[![CI](https://github.com/thelastinuit/cargo-fak/actions/workflows/ci.yml/badge.svg)](https://github.com/thelastinuit/cargo-fak/actions/workflows/ci.yml)

- Are you tired of your past persona making decisions that make no sense?
- Are you often asking yourself: why did I make that decision? 

Fear no more, Cargo Faaaaaak! is an ADR/RFC tool for your Rust projects. 
You don't need help, you are human, you are fine. 
What you need is better tools, Cargo Faaaaaak! aims to help you out.

#### Install

Install the CLI from crates.io:

```bash
$ cargo install cargo-fak
```

#### Usage

Create a new ADR with the binary directly:

```bash
$ cargo-fak adr a-title-for-your-record
```

Create a new RFC with the binary directly:

```bash
$ cargo-fak rfc a-title-for-your-record
```

Or via Cargo (Cargo runs the `cargo-fak` binary as the `fak` subcommand):

```bash
$ cargo fak adr a-title-for-your-record
$ cargo fak rfc a-title-for-your-record
```

By default, ADR records are written to `docs/adr` and RFC records to `docs/rfc`. Override with a flag or env var:

```bash
$ cargo-fak adr a-title-for-your-record --dir docs/adr
$ CARGO_FAK_ADR_DIR=docs/adr cargo-fak adr a-title-for-your-record
$ CARGO_FAK_RFC_DIR=docs/rfc cargo-fak rfc a-title-for-your-record
$ CARGO_FAK_DIR=docs/custom cargo-fak rfc a-title-for-your-record
```

Use a custom template (if the value is a file path, it is read; otherwise the value is used as the template):

```bash
$ cargo-fak adr a-title-for-your-record --template docs/adr/template.md
$ cargo-fak adr a-title-for-your-record --template "#### Context"
```

Preview the output path or print it for scripts:

```bash
$ cargo-fak adr a-title-for-your-record --dry-run --print-path
$ cargo-fak adr a-title-for-your-record --print-path
```

#### Usage examples script

Run a small script that exercises common ADR/RFC workflows in a temporary directory:

```bash
$ bash scripts/usage-examples.sh
```

And you will find your new record at `docs/adr/TIMESTAMP_a-title-for-your-record.md` or `docs/rfc/TIMESTAMP_a-title-for-your-record.md`.
