use generativelanguage_rs::common::api_client::{APIRequestClient, GenResponse};
use std::env;

async fn gpt(prompt_text: String) {
    let mut api_key = String::new();

    let key = "GOOGLE_API_KEY";
    match env::var(key) {
        Ok(val) => {
            api_key = val;
        },
        Err(e) => {
            println!("couldn't interpret {}: {}", key, e);
        }
    }

    let client = APIRequestClient::new(&api_key);
    match client.send_request(&prompt_text).await {
        Ok(response_text) => {
            // parse response
            let response = match serde_json::from_str::<GenResponse>(&response_text) {
                Ok(response) => response,
                Err(err) => {
                    eprintln!("Error occurred response_text: {}", err);
                    GenResponse {
                        candidates: Vec::new(),
                    }
                }
            };

            // print response
            for candidate in response.candidates {
                println!("output: {}", candidate.output);
                for safety_rating in candidate.safety_ratings {
                    println!(
                        "category: {}, probability: {}",
                        safety_rating.category, safety_rating.probability
                    );
                }
            }
        }
        Err(err) => {
            eprintln!("Error occurred: {}", err);
        }
    };
}
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let prompt = "Write a story about a magic backpack";
    gpt(prompt.to_string()).await;
    Ok(())
}
