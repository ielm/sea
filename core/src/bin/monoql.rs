use dotenvy::dotenv;
use monoql_core::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    run().await?;

    Ok(())
}
