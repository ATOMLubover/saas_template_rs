#[tokio::main]
async fn main() -> anyhow::Result<()> {
    saas_template_rs::run().await
}
