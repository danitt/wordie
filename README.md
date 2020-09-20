# Wordie
This tool scrapes term definitions from a given word list, and parses into both standardised JSON files, and optionally to Anki-compatible CSV files.

## Getting Started
1. Wordie uses the Velociraptor script runner, this must first be installed:
https://github.com/umbopepato/velociraptor

2. Create a file at the root of project, called 'words.csv' - this should contain a simple list of words to define

3. Run `vr start`

4. On completion, two files will be generated in the ./output directory
  - 'definitions.json' - definitions for given word list
  - 'failed.csv' - list of words where definitions could not be found

5. (optional) Produce an Anki-compatible export from the terms.json file, with `vr anki`