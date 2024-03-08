# Wordie
This tool scrapes term definitions from a given word list, and parses into both standardised JSON files, and optionally to Anki-compatible CSV files.

## Usage
1. Create a file at the root of project, called 'words.csv' - this should contain a simple list of words to define.
2. Run `cargo run define` to generate definitions.
3. Run `cargo run convert` to convert to Anki format.
