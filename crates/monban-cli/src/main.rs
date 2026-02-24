use std::{io::stdout, path::PathBuf};

use clap::{Parser as ClapParser, ValueEnum};
use serde_json::json;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use monban_core::Config;
use monban_service::{
    analysis::analyzer::WordAnalyzer,
    commands::analyze::{InputType, cmd_analyze},
};

#[derive(Clone, ValueEnum)]
enum CliInputType {
    Txt,
    Epub,
}

impl From<CliInputType> for InputType {
    fn from(t: CliInputType) -> Self {
        match t {
            CliInputType::Txt => InputType::Txt,
            CliInputType::Epub => InputType::Epub,
        }
    }
}

#[derive(ClapParser)]
struct Cli {
    #[arg(short, long, required = true)]
    input: PathBuf,
    #[arg(
        short,
        long = "type",
        required = true,
        value_enum,
        value_name = "txt|epub"
    )]
    ty: CliInputType,
    #[arg(short, long)]
    blacklist: Option<String>,
}

fn main() {
    init_logger();

    let config = Config::load();

    let cli = Cli::parse();

    let lexicon = cmd_analyze(&config, &cli.input, cli.ty.into(), cli.blacklist);

    let analyzer = WordAnalyzer::new(&config);

    let stats = analyzer.analyze(&lexicon);

    let output = json!({"stats": stats, "lexicon": lexicon});

    serde_json::to_writer_pretty(stdout(), &output).expect("Cannot export words");
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
