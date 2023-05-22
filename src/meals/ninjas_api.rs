#![allow(unused_doc_comments)]

use std::borrow::ToOwned;
use std::env;
use actix_web::http::StatusCode;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};


/// Struct to hold the API configuration
///
/// The struct contains the base URL and the API key and is populated from the .env file
struct APIConfig {
    base_url: String,
    api_key: String,
}

/// Struct to hold the nutrition information returned by the API
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

/// Implement the Clone trait for NutritionInfo
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

/// Implement the Default trait for NutritionInfo
impl Default for NutritionInfo {
    fn default() -> Self {
        NutritionInfo {
            name: String::new(),
            calories: 0.0,
            serving_size_g: 0.0,
            fat_total_g: 0.0,
            fat_saturated_g: 0.0,
            protein_g: 0.0,
            sodium_mg: 0.0,
            potassium_mg: 0.0,
            cholesterol_mg: 0.0,
            carbohydrates_total_g: 0.0,
            fiber_g: 0.0,
            sugar_g: 0.0,
        }
    }
}

/// Create a static instance of the API configuration ard populate it from the .env file
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

/// Function to get the nutrition information for a dish from the Ninjas API
/// ## Arguments
/// * `dish_name` - The name of the dish to get the nutrition information for
/// * `serving_size` - The serving size of the dish
/// ## Returns
/// * `Result<Vec<NutritionInfo>, StatusCode>` - A vector of NutritionInfo structs or an error code
pub async fn get_nutrition_info(dish_name: &str) -> Result<Vec<NutritionInfo>, StatusCode>{

    /// Create the URL for the API call
    let url = format!("{}{}", API_CONFIG.base_url, dish_name);
    /// Create a new [reqwest::Client](https://docs.rs/reqwest/0.11.3/reqwest/struct.Client.html)
    let client = reqwest::Client::new();
    /// Send the request to the API
    let response = client.get(&url)
        .header("X-Api-Key", API_CONFIG.api_key.clone())
        .send()
        .await
        .expect("Failed to send request");

    /// Check if the response status is 200
    ///
    /// If not, return the error code
    if response.status() != 200 {
        return Err(response.status());
    }

    /// Check if the response is empty
    ///
    /// If so, return an empty vector
    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => panic!("Error: {}", e),
    };

    /// Deserialize the JSON response into a vector of NutritionInfo structs
    ///
    /// If there is an error, panic
    let nutrition_info: Vec<NutritionInfo> = match serde_json::from_str(&body) {
        Ok(nutrition_info) => nutrition_info,
        Err(e) => panic!("Error: {}", e),
    };

    /// Return the vector of NutritionInfo structs
    Ok(nutrition_info)

}