# Wordie
This tool scrapes term definitions from a given word list, and parses into both standardised JSON files, and optionally to Anki-compatible CSV files.

## Usage
1. Create a file at the root of project, called 'words.csv' - this should contain a simple list of words to define.
2. Run `cargo run define` to generate definitions.
3. Run `cargo run convert` to convert to Anki format.

## Word of the day
`cargo run wod` logs into AnkiWeb (headless Chromium), picks three random cards from the `English` deck and posts them to a Discord channel with the definitions hidden behind spoilers. Posted words are recorded in `output/wod-history.json` and are not repeated until the deck runs out of unused words. Use `cargo run wod -- --dryrun` to print the message without posting (history is not updated).

### Setup
1. Install Chromium (set `CHROME_PATH` if it is not auto-detected, e.g. snap installs).
2. Copy `.env.example` to `.env` (gitignored) and fill in `ANKI_USER`, `ANKI_PASS`, `DISCORD_TOKEN` (bot token) and `DISCORD_CHANNEL_ID`. The bot needs permission to send messages in that channel. `.env` is read from the working directory.
3. `chmod 600 .env`

### Scheduling at 8:30am Brisbane time (systemd user timer)
Build a release binary first: `cargo build --release`. Replace `/home/you/wordie` with the real path.

`~/.config/systemd/user/wordie-wod.service`:
```ini
[Unit]
Description=Wordie word of the day

[Service]
Type=oneshot
WorkingDirectory=/home/you/wordie
EnvironmentFile=/home/you/wordie/.env
ExecStart=/home/you/wordie/target/release/wordie wod
```

`~/.config/systemd/user/wordie-wod.timer`:
```ini
[Unit]
Description=Daily Wordie word of the day

[Timer]
OnCalendar=*-*-* 08:30:00 Australia/Brisbane
Persistent=true

[Install]
WantedBy=timers.target
```

Then:
```sh
systemctl --user daemon-reload
systemctl --user enable --now wordie-wod.timer
loginctl enable-linger $USER   # run even when not logged in
systemctl --user list-timers wordie-wod.timer
systemctl --user start wordie-wod.service   # test run
journalctl --user -u wordie-wod.service
```

`EnvironmentFile` takes plain `KEY=value` lines (no `export`, no quotes needed).

### Scheduling with cron
Cron has no `.env` loading of its own, but the binary reads `.env` from its working directory, so `cd` first. Set the timezone with `CRON_TZ` (supported by cronie and Debian/Ubuntu cron):
```cron
CRON_TZ=Australia/Brisbane
30 8 * * * cd /home/you/wordie && ./target/release/wordie wod >> output/wod.log 2>&1
```
