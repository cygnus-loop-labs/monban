# monban

Japanese text difficulty analyzer for language learners.
Identifies unknown vocabulary and kanji, assesses reading readiness, and generates a personalized vocabulary deck.

## Planned Features

- **Coverage analysis**: Calculate known word/kanji percentage and unknown word/kanji density
- **Multi-source vocabulary**: Integration with different sources (SRS, vocab lists) to determine known words/kanji.
- **Difficulty rating**: Easy (98%+) → Too Hard (<85%)
- **Frequency-based recommendations**: Learn the 20-50 most frequent unknown words to reach 95%+ coverage
- **Text input**: Plain text, EPUB files, ...
- **Smart filtering**: Excludes particles, grammar, numbers, proper nouns — focuses on content words

## Workflow

1. Input Japanese text
2. Get difficulty assessment (coverage %, density, unknown word/kanji count)
3. Receive prioritized vocabulary list (sorted by frequency in text)
4. Study high-impact words before reading

## Status

Active development.

## Credit

This tool uses the JMdict/EDICT and KANJIDIC dictionary files. These files are the property of the Electronic Dictionary Research and Development Group, and are used in conformance with the Group's licence.
JLPT vocabulary data by Jonathan Waller (https://www.tanos.co.uk/jlpt/) - License: Creative Commons BY — https://creativecommons.org/licenses/by/4.0/
