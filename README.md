# RustLens

RustLens is a Rust-based CLI tool for analyzing database tables through a TOML configuration file.

The idea is to describe a table in a config file, then let RustLens load that config and run different analysis flows against the data.

## Status

Early work in progress.

Current progress includes:

- CLI structure with `clap`
- TOML config loading
- custom config error handling
- service/module structure for analysis commands

## Goals

RustLens is intended to:

- connect to a database using config
- describe a table through TOML
- analyze the table based on configured rules
- support different analysis depths
- export results, for example to CSV

## Project structure

```text
src/
├── main.rs
├── lib.rs
├── cli.rs
├── db/
│   ├── mod.rs
│   ├── config.rs
│   └── errors.rs
└── services/
    ├── mod.rs
    └── analysis.rs
```

## Example usage

```bash
cargo run -- analyze --config config.toml
```

## Configuration

RustLens uses a TOML config file.

Example:

```toml
[database]
url = "postgres://user:password@localhost:5432/logs_db"

[table]
name = "application_logs"

[table.columns]
timestamp = "created_at"
level = "severity"
message = "log_message"

[analysis]
level = "quick"

[output]
format = "csv"
path = "reports/log_analysis.csv"
```

## Planned features

- database connection handling
- configurable table/column mapping
- analysis levels
- CSV output
- stronger validation
- tests for config parsing and error cases