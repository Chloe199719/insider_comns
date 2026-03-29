#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    insider_comms::run().await?;
    Ok(())
}
