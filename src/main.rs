mod cli;
mod commands;
mod r#const;
mod modules;

#[tokio::main]
async fn main() {
    cli::prompt().await;
}
