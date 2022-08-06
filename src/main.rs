use showller::tmdb;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = tmdb::Client::new(&std::env::var("API").unwrap(), Some("zh-CN".to_string()))?;
    let show = client.get_show(129195).await?;

    println!("{:#?}", show);

    Ok(())
}
