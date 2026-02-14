use std::{fs, path::PathBuf};

use clap::Parser as ClapParser;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use monban_parser::Parser;

#[derive(ClapParser)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,
}

fn main() {
    init_logger();

    let parser = Parser::new();

    let cli = Cli::parse();

    let content = fs::read_to_string(cli.input).unwrap();

    let words = parser.load_text(&content);

    for word in words {
        println!("{:?}", word);
    }
}

fn init_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_writer(std::io::stderr)
        .init();

    tracing::info!(
        "Logger initialized (RUST_LOG={:?})",
        std::env::var("RUST_LOG")
    );
}
