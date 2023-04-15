use std::borrow::ToOwned;
use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

struct APIConfig {
    base_url: String,
    api_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct NutritionInfo {
    pub name: String,
    pub calories: f32,
    pub serving_size_g: f32,
    pub fat_total_g: f32,
    pub fat_saturated_g: f32,
    pub protein_g: f32,
    pub sodium_mg: f32,
    pub potassium_mg: f32,
    pub cholesterol_mg: f32,
    pub carbohydrates_total_g: f32,
    pub fiber_g: f32,
    pub sugar_g: f32,
}

lazy_static! {
    static ref API_CONFIG: APIConfig = {
        dotenv().expect("Failed to read .env file");
        let base_url = env::var("NINJAS_API_BASE_URL").expect("NINJA_API_BASE_URL must be set").to_owned();
        let api_key = env::var("NINJAS_API_KEY").expect("NINJA_API_KEY must be set").to_owned();
        APIConfig {
            base_url,
            api_key,
        }
    };
}


pub async fn get_nutrition_info(dish_name: &str) -> Vec<NutritionInfo>{
    let url = format!("{}{}", API_CONFIG.base_url, dish_name);
    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("X-Api-Key", API_CONFIG.api_key.clone())
        .send()
        .await
        .expect("Failed to send request");

    // get response body string
    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => panic!("Error: {}", e),
    };

    let nutrition_info: Vec<NutritionInfo> = match serde_json::from_str(&body) {
        Ok(nutrition_info) => nutrition_info,
        Err(e) => panic!("Error: {}", e),
    };

    nutrition_info

}


