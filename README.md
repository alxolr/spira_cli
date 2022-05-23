## Pre-requisites

You should have rust and cargo installed.

## Install

```bash
cargo install spira_cli
```

## Configuration

You should export the following envars in your system

```bash
export SPIRA_API_KEY="{geneated_api_key_including_brackets}"
export SPIRA_USERNAME="YourUsername"
export SPIRA_API_URL="https://{company}.spiraservice.net/Services/v6_0/RestService.svc"
export SPIRA_BASE_URL="https://{company}.spiraservice.net"
```

## Usage

```bash
➜  ~ spira_cli
Spira Cli

USAGE:
    spira_cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help           Prints this message or the help of the given subcommand(s)
    incident       Incident Cli
    requirement    Requirement Cli
    task           Tasks Cli
    user           Users Cli
```
