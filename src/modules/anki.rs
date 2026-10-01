use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::Page;
use futures::StreamExt;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;

use super::config::AnkiConfig;

const LOGIN_URL: &str = "https://ankiweb.net/account/login";
const SEARCH_URL: &str = "https://ankiweb.net/search";
const DECK_NAME: &str = "English";
const MAX_SEARCHES: usize = 40;
const MIN_NON_EMPTY_SEARCHES: usize = 4;
const RESULT_TIMEOUT_MS: u64 = 8000;
const POLL_INTERVAL_MS: u64 = 250;

const EXTRACT_CARDS_JS: &str = r#"() => {
    const rows = Array.from(document.querySelectorAll('tr.light-bottom-border'));
    return rows.map(row => {
        const tds = row.querySelectorAll('td');
        if (tds.length < 2) return null;
        const content = tds[1].innerText.trim();
        const slashIdx = content.indexOf(' / ');
        if (slashIdx === -1) return null;
        return {
            word: content.slice(0, slashIdx).trim(),
            definition: content.slice(slashIdx + 3).trim()
        };
    }).filter(Boolean);
}"#;

#[derive(Debug, Clone, Deserialize)]
pub struct Card {
    pub word: String,
    pub definition: String,
}

pub struct Selection {
    pub cards: Vec<Card>,
    pub history_reset: bool,
}

type AnkiResult<T> = Result<T, Box<dyn std::error::Error>>;

async fn pause(milliseconds: u64) {
    sleep(Duration::from_millis(milliseconds)).await;
}

async fn click_button(page: &Page, label: &str) -> AnkiResult<()> {
    let script = format!(
        "() => {{ const b = Array.from(document.querySelectorAll('button')).find(b => b.innerText.trim() === '{}'); if (b) b.click(); return !!b; }}",
        label
    );
    let clicked = page.evaluate_function(script).await?.into_value::<bool>()?;
    if clicked {
        Ok(())
    } else {
        Err(format!("Button '{}' not found", label).into())
    }
}

async fn set_input(page: &Page, selector: &str, value: &str) -> AnkiResult<()> {
    let script = format!(
        "() => {{ const input = document.querySelector({}); if (!input) return false; const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value').set; setter.call(input, {}); input.dispatchEvent(new Event('input', {{ bubbles: true }})); return true; }}",
        serde_json::to_string(selector)?,
        serde_json::to_string(value)?
    );
    for _ in 0..(RESULT_TIMEOUT_MS / POLL_INTERVAL_MS) {
        pause(POLL_INTERVAL_MS).await;
        if page
            .evaluate_function(script.clone())
            .await?
            .into_value::<bool>()?
        {
            return Ok(());
        }
    }
    Err(format!("Input '{}' not found", selector).into())
}

async fn log_in(page: &Page, config: &AnkiConfig) -> AnkiResult<()> {
    page.goto(LOGIN_URL).await?;
    set_input(page, "input[placeholder='Email']", &config.username).await?;
    set_input(page, "input[placeholder='Password']", &config.password).await?;
    click_button(page, "Log In").await?;
    for _ in 0..(RESULT_TIMEOUT_MS / POLL_INTERVAL_MS) {
        pause(POLL_INTERVAL_MS).await;
        let url = page.url().await?.unwrap_or_default();
        if !url.contains("/account/login") {
            return Ok(());
        }
    }
    Err("AnkiWeb login failed, check ANKI_USER and ANKI_PASS".into())
}

async fn read_cards(page: &Page) -> AnkiResult<Vec<Card>> {
    Ok(page
        .evaluate_function(EXTRACT_CARDS_JS)
        .await?
        .into_value::<Vec<Card>>()?)
}

fn signature(cards: &[Card]) -> String {
    cards
        .iter()
        .map(|c| c.word.as_str())
        .collect::<Vec<_>>()
        .join("|")
}

async fn submit_search(page: &Page, query: &str) -> AnkiResult<()> {
    set_input(page, "input[type=text]", query).await?;
    click_button(page, "Search").await
}

/*
AnkiWeb caps search results at 100 rows with no pagination, so the deck is sampled through narrow queries instead of listed in full. The search only resolves after the results table changes, so poll until it differs from the previous results.
*/
async fn search(page: &Page, query: &str, previous: &[Card]) -> AnkiResult<Vec<Card>> {
    submit_search(page, query).await?;
    let before = signature(previous);
    let mut cards = Vec::new();
    for _ in 0..(RESULT_TIMEOUT_MS / POLL_INTERVAL_MS) {
        pause(POLL_INTERVAL_MS).await;
        cards = read_cards(page).await?;
        if signature(&cards) != before {
            break;
        }
    }
    Ok(cards)
}

fn shuffled_prefixes() -> Vec<String> {
    let letters: Vec<char> = ('a'..='z').collect();
    let mut prefixes: Vec<String> = letters
        .iter()
        .flat_map(|first| {
            letters
                .iter()
                .map(move |second| format!("{}{}", first, second))
        })
        .collect();
    prefixes.shuffle(&mut rand::rng());
    prefixes
}

fn prefix_query(prefix: &str) -> String {
    format!("deck:{} front:{}*", DECK_NAME, prefix)
}

async fn collect_pool(
    page: &Page,
    used: &HashSet<String>,
    count: usize,
) -> AnkiResult<(Vec<Card>, Vec<Card>)> {
    let mut unused: Vec<Card> = Vec::new();
    let mut everything: Vec<Card> = Vec::new();
    let mut previous: Vec<Card> = Vec::new();
    let mut non_empty = 0;
    for prefix in shuffled_prefixes().into_iter().take(MAX_SEARCHES) {
        let cards = search(page, &prefix_query(&prefix), &previous).await?;
        previous = cards.clone();
        if cards.is_empty() {
            continue;
        }
        non_empty += 1;
        unused.extend(cards.iter().filter(|c| !used.contains(&c.word)).cloned());
        everything.extend(cards);
        if non_empty >= MIN_NON_EMPTY_SEARCHES && unused.len() >= count {
            break;
        }
    }
    Ok((unused, everything))
}

async fn pick_from_pool(
    page: &Page,
    used: &HashSet<String>,
    count: usize,
) -> AnkiResult<Selection> {
    page.goto(SEARCH_URL).await?;
    pause(1500).await;
    let (mut unused, mut everything) = collect_pool(page, used, count).await?;
    let history_reset = unused.len() < count;
    let pool = if history_reset {
        &mut everything
    } else {
        &mut unused
    };
    if pool.is_empty() {
        return Err("No cards found in the Anki deck".into());
    }
    pool.shuffle(&mut rand::rng());
    pool.truncate(count);
    Ok(Selection {
        cards: pool.clone(),
        history_reset,
    })
}

fn profile_dir() -> PathBuf {
    std::env::temp_dir().join(format!("wordie-chrome-{}", std::process::id()))
}

async fn launch_browser(config: &AnkiConfig) -> AnkiResult<Browser> {
    let mut builder = BrowserConfig::builder()
        .no_sandbox()
        .user_data_dir(profile_dir());
    if let Some(path) = &config.chrome_path {
        builder = builder.chrome_executable(path);
    }
    let (browser, mut handler) = Browser::launch(builder.build()?).await?;
    tokio::spawn(async move { while handler.next().await.is_some() {} });
    Ok(browser)
}

pub async fn select_cards(
    config: &AnkiConfig,
    used: &HashSet<String>,
    count: usize,
) -> AnkiResult<Selection> {
    let mut browser = launch_browser(config).await?;
    let outcome = async {
        let page = browser.new_page("about:blank").await?;
        log_in(&page, config).await?;
        pick_from_pool(&page, used, count).await
    }
    .await;
    browser.close().await.ok();
    browser.wait().await.ok();
    std::fs::remove_dir_all(profile_dir()).ok();
    outcome
}
