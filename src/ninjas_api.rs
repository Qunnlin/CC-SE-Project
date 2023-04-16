use std::borrow::ToOwned;
use std::env;
use actix_web::http::StatusCode;
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
    pub calories: f64,
    pub serving_size_g: f64,
    pub fat_total_g: f64,
    pub fat_saturated_g: f64,
    pub protein_g: f64,
    pub sodium_mg: f64,
    pub potassium_mg: f64,
    pub cholesterol_mg: f64,
    pub carbohydrates_total_g: f64,
    pub fiber_g: f64,
    pub sugar_g: f64,
}

// Clone for Nutrition Info
impl Clone for NutritionInfo {
    fn clone(&self) -> Self {
        NutritionInfo {
            name: self.name.clone(),
            calories: self.calories,
            serving_size_g: self.serving_size_g,
            fat_total_g: self.fat_total_g,
            fat_saturated_g: self.fat_saturated_g,
            protein_g: self.protein_g,
            sodium_mg: self.sodium_mg,
            potassium_mg: self.potassium_mg,
            cholesterol_mg: self.cholesterol_mg,
            carbohydrates_total_g: self.carbohydrates_total_g,
            fiber_g: self.fiber_g,
            sugar_g: self.sugar_g,
        }
    }
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


pub async fn get_nutrition_info(dish_name: &str) -> Result<Vec<NutritionInfo>, StatusCode>{


    let url = format!("{}{}", API_CONFIG.base_url,dish_name);
    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("X-Api-Key", API_CONFIG.api_key.clone())
        .send()
        .await
        .expect("Failed to send request");


    if response.status() != 200 {
        return Err(response.status());
    }

    // Check if body is empty
    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => panic!("Error: {}", e),
    };

    let nutrition_info: Vec<NutritionInfo> = match serde_json::from_str(&body) {
        Ok(nutrition_info) => nutrition_info,
        Err(e) => panic!("Error: {}", e),
    };

    Ok(nutrition_info)

}


