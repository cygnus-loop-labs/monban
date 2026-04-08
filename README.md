# monban

Japanese text difficulty analyzer for language learners.
Identifies unknown vocabulary and kanji, assesses reading difficulty, and generates a personalized vocabulary deck.

## Features
- **Text input**: Parse japanese text from multiple format (plain text, EPUB), excluding non content words
- **Known vocabulary**: Integrate with different sources (SRS, vocab lists) to determine known words/kanji.
- **Coverage analysis**: Calculate known word/kanji percentage, generate list of unknown words sorted by frequency
- **Integrations**: Anki Connect, JMdict, JLPT word list

## Build && Run
- cargo install tauri-cli
- cargo tauri dev

## Status

Active development.

## Credit

- This tool uses the JMdict/EDICT and KANJIDIC dictionary files. These files are the property of the Electronic Dictionary Research and Development Group, and are used in conformance with the Group's licence.
- JLPT vocabulary data by Jonathan Waller (https://www.tanos.co.uk/jlpt/) - License: Creative Commons BY — https://creativecommons.org/licenses/by/4.0/

