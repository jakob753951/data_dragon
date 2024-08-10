mod models;
mod api;

use color_eyre::Result;
use crate::api::Api;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let api = Api;
    let champions = api.get_champions().await?;

    println!("{:#?}", champions);

    Ok(())
}
