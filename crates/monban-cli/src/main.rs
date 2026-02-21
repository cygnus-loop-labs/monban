use std::{collections::HashSet, path::PathBuf};

use clap::{Parser as ClapParser, ValueEnum};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use monban_core::{Config, Deck, Lexicon};
use monban_parser::{DeckLoader as _, JLPTDeckLoader, Parser, PlainDeckLoader, WKDeckLoader};

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

    check_words(&mut words, decks);

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
