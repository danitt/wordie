use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::Page;
use futures::StreamExt;
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;

use super::config::AnkiConfig;

const LOGIN_URL: &str = "https://ankiweb.net/account/login";
const SEARCH_URL: &str = "https://ankiweb.net/search";
const SEARCH_QUERY: &str = "deck:English";

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

type AnkiResult<T> = Result<T, Box<dyn std::error::Error>>;

async fn pause(seconds: u64) {
    sleep(Duration::from_secs(seconds)).await;
}

async fn log_in(page: &Page, config: &AnkiConfig) -> AnkiResult<()> {
    page.goto(LOGIN_URL).await?;
    pause(2).await;
    page.find_element("input[placeholder='Email']")
        .await?
        .click()
        .await?
        .type_str(&config.username)
        .await?;
    page.find_element("input[placeholder='Password']")
        .await?
        .click()
        .await?
        .type_str(&config.password)
        .await?
        .press_key("Enter")
        .await?;
    pause(4).await;
    Ok(())
}

async fn search_cards(page: &Page) -> AnkiResult<Vec<Card>> {
    page.goto(SEARCH_URL).await?;
    pause(2).await;
    page.find_element("input[type='text']")
        .await?
        .click()
        .await?
        .type_str(SEARCH_QUERY)
        .await?
        .press_key("Enter")
        .await?;
    pause(4).await;
    let cards = page
        .evaluate_function(EXTRACT_CARDS_JS)
        .await?
        .into_value::<Vec<Card>>()?;
    Ok(cards)
}

async fn launch_browser(config: &AnkiConfig) -> AnkiResult<Browser> {
    let mut builder = BrowserConfig::builder().no_sandbox();
    if let Some(path) = &config.chrome_path {
        builder = builder.chrome_executable(path);
    }
    let (browser, mut handler) = Browser::launch(builder.build()?).await?;
    tokio::spawn(async move { while handler.next().await.is_some() {} });
    Ok(browser)
}

pub async fn fetch_cards(config: &AnkiConfig) -> AnkiResult<Vec<Card>> {
    let mut browser = launch_browser(config).await?;
    let outcome = async {
        let page = browser.new_page("about:blank").await?;
        log_in(&page, config).await?;
        search_cards(&page).await
    }
    .await;
    browser.close().await.ok();
    browser.wait().await.ok();
    outcome
}
