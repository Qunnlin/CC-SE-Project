#![allow(unused_doc_comments)]

/// Module that handles communication with the Diets Microservice
/// Actix imports
use actix_web::http::StatusCode;
use dotenv::dotenv;

/// Misc imports
use reqwest::get;



/*
======================= GET /diet/{name} =======================
 */
/// # Get Diet by Name
/// Function that gets a diet by name from the Diets Microservice
/// ## Arguments
/// * diet_name - The name of the diet to get
/// ## Returns
/// * A [Result] containing a [Diet] if the diet was found, or an [reqwest::Error] if the request failed
pub async fn get_diet_by_name(diet_name: &str) -> Result<String, reqwest::StatusCode> {
    dotenv().ok();
    let diets_url = std::env::var("DIET_SERVICE_URL").expect("DIET_SERVICE_URL must be set");
    /// Send a GET request to the Diets Microservice
    let res = get(format!("{}/diets/{}", diets_url, diet_name)).await;
    match res {
        Ok(res) => {
            if res.status().is_success() {
                let data = res.text().await.unwrap();
                Ok(data)
            } else {
                Err(res.status())

            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}