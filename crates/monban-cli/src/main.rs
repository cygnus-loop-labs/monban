use std::{io::stdout, path::PathBuf};

use anyhow::Result;
use clap::{Parser as ClapParser, Subcommand, ValueEnum};
use serde_json::json;

use monban_core::Config;
use monban_service::{
    analysis::analyzer::WordAnalyzer,
    commands::analyze::cmd_analyze,
    parsing::{InputType, deck::AnkiDeckLoader},
    util::init_logger,
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
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
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
    },
    Anki {},
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();

    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { input, ty } => analyze(input, ty),
        Commands::Anki {} => anki().await,
    }
}

fn analyze(input: PathBuf, ty: CliInputType) -> Result<()> {
    let config = Config::load();

    let lexicon = cmd_analyze(&config, input, ty.into(), |p| {
        tracing::info!("Analysis progress: {}", p);
    })?;

    let analyzer = WordAnalyzer::new(&config);

    let stats = analyzer.analyze(&lexicon);

    let output = json!({"stats": stats, "lexicon": lexicon});

    serde_json::to_writer_pretty(stdout(), &output).expect("Cannot export words");

    Ok(())
}

async fn anki() -> Result<()> {
    let config = Config::load();

    let decks = AnkiDeckLoader::list_decks(&config).await?;

    tracing::info!("Decks: {}", decks.len());
    for deck in &decks {
        tracing::info!("Deck: {:?}", deck);
    }

    Ok(())
}
