use color_eyre::Result;

use crate::models::*;

pub struct Api;

impl Api {
    pub async fn get_champions(&self) -> Result<Vec<Champion>> {
        let champions = reqwest::get("https://ddragon.leagueoflegends.com/cdn/14.15.1/data/en_US/champion.json")
            .await?
            .json::<Response>()
            .await?
            .data
            .iter()
            .map(|a| a.1.clone())
            .collect::<Vec<Champion>>();

        Ok(champions)
    }
}