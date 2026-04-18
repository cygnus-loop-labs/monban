use std::{io::stdout, path::PathBuf};

use anyhow::Result;
use clap::{Parser as ClapParser, Subcommand, ValueEnum};
use lindera::{
    dictionary::load_dictionary, mode::Mode, segmenter::Segmenter, tokenizer::Tokenizer,
};
use serde_json::json;

use monban_core::{Config, DeckConfig};
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
    },
    Anki {
        #[arg(required = false)]
        deck: Option<String>,
    },
    Token {
        txt: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();

    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { input } => analyze(input).await,
        Commands::Anki { deck } => anki(deck).await,
        Commands::Token { txt } => token(txt),
    }
}

async fn analyze(input: PathBuf) -> Result<()> {
    let config = Config::load();

    let lexicon = cmd_analyze(&config, input, |p| {
        tracing::info!("Analysis progress: {}", p);
    })
    .await?;

    let analyzer = WordAnalyzer::new(&config);

    let stats = analyzer.analyze(&lexicon);
    let total = stats.words.count as usize;

    let output = json!({"stats": stats, "lexicon": lexicon});

    serde_json::to_writer_pretty(stdout(), &output).expect("Cannot export words");

    tracing::info!(
        "Processed words: {}, skipped: {}, filtered: {}, blacklisted: {}, lexicon: {}",
        lexicon.tokens,
        lexicon.skipped,
        lexicon.filtered,
        lexicon.blacklisted,
        total
    );

    assert_eq!(
        lexicon.tokens,
        lexicon.skipped + lexicon.blacklisted + lexicon.filtered + total
    );

    Ok(())
}

async fn anki(deck: Option<String>) -> Result<()> {
    let config = Config::load();

    let anki = AnkiDeckLoader::new(&config);

    match deck {
        Some(deck) => {
            for deck_config in config.decks {
                if let DeckConfig::Anki {
                    name,
                    word,
                    reading,
                    meaning,
                } = deck_config
                    && name == deck
                {
                    let deck = anki.get_deck(&deck, &word, &reading, &meaning).await?;
                    tracing::info!("Deck: {}", deck.words_len());
                    tracing::info!("Deck: {:?}", deck);
                }
            }
        }
        None => {
            let decks = anki.list_decks().await?;
            tracing::info!("Decks: {}", decks.len());
            for deck in &decks {
                tracing::info!("Deck: {:?}", deck);
            }

            anki.get_models().await?;
        }
    }
    Ok(())
}

fn token(txt: String) -> Result<()> {
    let config = Config::load();

    let ipadic = load_dictionary(&config.parser.dictionary).unwrap();
    let tokenizer = Tokenizer::new(Segmenter::new(Mode::Normal, ipadic, None));

    let tokens = tokenizer.tokenize(&txt)?;

    for mut token in tokens {
        let surface = token.surface.to_string();
        let details = token.details().clone();
        println!("token: {:?}, details: {:?}", surface, details);
    }

    Ok(())
}
