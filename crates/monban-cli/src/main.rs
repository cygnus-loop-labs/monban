use std::path::PathBuf;

use clap::{Parser as ClapParser, ValueEnum};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use monban_core::{Config, Deck, Lexicon};
use monban_parser::{Parser, PlainDeckLoader};

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
    sort: bool,
    #[arg(
        short,
        long = "type",
        required = true,
        value_enum,
        value_name = "txt|epub"
    )]
    ty: InputType,
}

fn main() {
    init_logger();

    let config = Config::load();

    let parser = Parser::new(&config);

    let cli = Cli::parse();

    let mut words = match cli.ty {
        InputType::Txt => parser.load_text(cli.input),
        InputType::Epub => parser.load_epub(cli.input),
    };

    let decks = config
        .user_decks
        .decks
        .iter()
        .map(PlainDeckLoader::load)
        .collect::<Vec<Deck>>();

    check_words(&mut words, &decks);

    dump_words(&words, cli.sort);
}

fn check_words(words: &mut Lexicon, decks: &[Deck]) {
    for word in &mut words.iter_mut() {
        for deck in decks {
            deck.check(word);
        }
    }
}

fn dump_words(words: &Lexicon, sort: bool) {
    let mut results: Vec<_> = words.iter().collect();
    if sort {
        results.sort_by_key(|w| std::cmp::Reverse(w.count));
    }

    for word in &mut results {
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
