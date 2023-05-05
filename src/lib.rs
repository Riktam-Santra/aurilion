use models::{champion_details::ChampionDetails, champions::Champions, items::Items};
mod models;

pub async fn get_versions() -> Result<Vec<String>, reqwest::Error> {
    match reqwest::get("https://ddragon.leagueoflegends.com/api/versions.json").await {
        Ok(response) => {
            let response_text = response.text().await.unwrap();
            let data_array: Vec<String> = serde_json::from_str(&response_text).unwrap();
            Ok(data_array)
        }
        Err(e) => Err(e),
    }
}

pub async fn get_all_champions(version: String) -> Result<Champions, reqwest::Error> {
    match reqwest::get(format!(
        "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion.json",
        version
    ))
    .await
    {
        Ok(response) => {
            let response_text = response.text().await.unwrap();
            let parsed_data: Champions = serde_json::from_str(&response_text).unwrap();
            Ok(parsed_data)
        }
        Err(e) => Err(e),
    }
}

pub async fn get_single_champion(
    name: String,
    version: String,
) -> Result<ChampionDetails, reqwest::Error> {
    let url = format!(
        "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion/{}.json",
        version, name
    );
    println!("{}", url);
    match reqwest::get(url).await {
        Ok(response) => {
            let response_text = response.text().await.unwrap();
            let parsed_data: ChampionDetails = serde_json::from_str(&response_text).unwrap();
            Ok(parsed_data)
        }
        Err(e) => Err(e),
    }
}
pub async fn get_all_items(version: String) -> Result<Items, reqwest::Error> {
    let url = format!(
        "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/item.json",
        version
    );
    match reqwest::get(url).await {
        Ok(response) => {
            let response_text = response.text().await.unwrap();
            let parsed_data: Items = serde_json::from_str(&response_text).unwrap();
            Ok(parsed_data)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests;
