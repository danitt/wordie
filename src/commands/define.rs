use crate::modules::define::generate_definitions;

pub async fn run() {
    match generate_definitions().await {
        Ok(_) => println!("⚡ Definitions generated successfully!"),
        Err(e) => eprintln!("💥 Error generating definitions: {}", e),
    }
}
