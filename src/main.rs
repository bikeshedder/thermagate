#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    altherma_gateway::gateway::main().await
}