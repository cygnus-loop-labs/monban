use std::{fs, path::PathBuf};

use clap::Parser as ClapParser;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use monban_core::{Config, Deck, Lexicon};
use monban_parser::{Parser, PlainDeckLoader};

#[derive(ClapParser)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,
}

fn main() {
    init_logger();

    let config = Config::load();

    let parser = Parser::new(&config);

    let cli = Cli::parse();

    let content = fs::read_to_string(cli.input).unwrap();

    let mut words = parser.load_text(&content);

    let decks = config
        .user_decks
        .decks
        .iter()
        .map(PlainDeckLoader::load)
        .collect::<Vec<Deck>>();

    check_words(&mut words, &decks);
}

fn check_words(words: &mut Lexicon, decks: &[Deck]) {
    let mut results: Vec<_> = words.iter_mut().collect();
    results.sort_by_key(|w| std::cmp::Reverse(w.count));

    for word in &mut results {
        for deck in decks {
            deck.check(word);
        }

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
