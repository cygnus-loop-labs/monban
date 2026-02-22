use std::{collections::HashSet, io::stdout, path::PathBuf};

use clap::{Parser as ClapParser, ValueEnum};
use serde_json::json;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use monban_core::{Config, Deck};
use monban_service::{
    analysis::analyzer::WordAnalyzer,
    parsing::{DeckLoader as _, JLPTDeckLoader, Parser, PlainDeckLoader, WKDeckLoader},
};

#[derive(Clone, ValueEnum)]
enum InputType {
    Txt,
    Epub,
}

#[derive(ClapParser)]
struct Cli {
    #[arg(short, long, required = true)]
    input: PathBuf,
    #[arg(short, long)]
    #[arg(
        short,
        long = "type",
        required = true,
        value_enum,
        value_name = "txt|epub"
    )]
    ty: InputType,
    #[arg(short, long)]
    blacklist: Option<String>,
}

fn main() {
    init_logger();

    let config = Config::load();

    let parser = Parser::new(&config);

    let cli = Cli::parse();

    let blacklist = if let Some(blacklist) = &cli.blacklist {
        parser.load_blacklist(blacklist)
    } else {
        HashSet::default()
    };

    let mut words = match cli.ty {
        InputType::Txt => parser.load_text(cli.input, &blacklist),
        InputType::Epub => parser.load_epub(cli.input, &blacklist),
    };

    let decks = &mut config
        .decks
        .iter()
        .map(|(name, params)| match params.ty.as_str() {
            "plain" => PlainDeckLoader::load(name.to_string(), &params.file, &config),
            "wk" => WKDeckLoader::load(name.to_string(), &params.file, &config),
            "jlpt" => JLPTDeckLoader::load(name.to_string(), &params.file, &config),
            _ => unimplemented!(),
        })
        .collect::<Vec<Deck>>();

    for word in words.iter_mut() {
        for deck in decks.iter_mut() {
            deck.check(word);
        }
    }

    let analyzer = WordAnalyzer::new(&config);

    let stats = analyzer.analyze(&words);

    let output = json!({"stats": stats, "lexicon": words});

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
