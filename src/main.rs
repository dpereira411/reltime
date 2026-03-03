use chrono::Utc;
use clap::Parser;
use reltime::{format_relative, format_relative_exact, parse_timestamp};

#[derive(Debug, Parser)]
#[command(
    name = "reltime",
    about = "Convert a timestamp to a relative time string",
    after_help = "Examples:\n  reltime \"2025-10-01T12:00:00Z\"\n  reltime \"2025-10-01 12:00:00\"\n  reltime \"2025-10-01\"\n  reltime --exact \"2025-10-01T12:00:00Z\""
)]
struct Cli {
    /// Print a multi-unit precise relative duration (up to 3 units)
    #[arg(short = 'e', long = "exact")]
    exact: bool,

    /// Input timestamp, e.g. RFC3339 or common local date-time formats
    timestamp: String,
}

fn main() {
    let args = Cli::parse();
    match parse_timestamp(&args.timestamp) {
        Ok(target) => {
            let now = Utc::now();
            let out = if args.exact {
                format_relative_exact(target, now)
            } else {
                format_relative(target, now)
            };
            println!("{out}");
        }
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(2);
        }
    }
}
