use std::{io::stdout, path::PathBuf};

use anyhow::Result;
use clap::{Parser as ClapParser, ValueEnum};
use serde_json::json;

use monban_core::Config;
use monban_service::{
    analysis::analyzer::WordAnalyzer, commands::analyze::cmd_analyze, parsing::InputType,
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

fn main() -> Result<()> {
    init_logger();

    let config = Config::load();

    let cli = Cli::parse();

    let lexicon = cmd_analyze(&config, &cli.input, cli.ty.into())?;

    let analyzer = WordAnalyzer::new(&config);

    let stats = analyzer.analyze(&lexicon);

    let output = json!({"stats": stats, "lexicon": lexicon});

    serde_json::to_writer_pretty(stdout(), &output).expect("Cannot export words");

    Ok(())
}
