# reltime

`reltime` is a small Rust CLI that converts a timestamp into a human-friendly relative time string.

Examples:
- `5 mins ago`
- `7 months ago`
- `in 4 years`

## Features

- Parses RFC3339/ISO-8601 timestamps with timezone offsets
- Accepts common local date-time formats
- Outputs compact relative strings for past and future dates
- Supports `--exact` for precise multi-unit relative output
- Uses predictable floor rounding with fixed month/year durations

## Install

Build from source:

```bash
cargo build --release
```

Binary location:

```bash
./target/release/reltime
```

## Usage

```bash
reltime [--exact] <timestamp>
```

Examples:

```bash
reltime "2025-10-01T12:00:00Z"
reltime "2025-10-01 12:00:00"
reltime "2025-10-01"
reltime --exact "2025-10-01T12:00:00Z"
```

## Supported Input Formats

- `YYYY-MM-DDTHH:MM:SSZ` / RFC3339 with timezone offset
- `YYYY-MM-DD HH:MM:SS` (interpreted in local timezone)
- `YYYY-MM-DDTHH:MM:SS` (interpreted in local timezone)
- `YYYY-MM-DD HH:MM` (interpreted in local timezone)
- `YYYY-MM-DD` (local midnight)

## Output Rules

- Past: `{n} {unit} ago`
- Future: `in {n} {unit}`
- Units: `sec(s)`, `min(s)`, `hr(s)`, `day(s)`, `month(s)`, `year(s)`
- Month and year are fixed durations:
  - `1 month = 30 days`
  - `1 year = 365 days`

### `--exact` Mode

- Produces a precise multi-unit breakdown (up to 3 non-zero units)
- Uses abbreviated units (`years`, `months`, `days`, `hrs`, `mins`, `secs`)
- Examples:
  - `5 months 3 days 2 hrs ago`
  - `in 1 year 1 month 1 day`
  - `0 secs ago` (when timestamp equals current time)

## Development

Run tests:

```bash
cargo test
```

Run locally:

```bash
cargo run -- "2025-10-01T12:00:00Z"
```
