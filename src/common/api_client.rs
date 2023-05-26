use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub output: String,
    pub safety_ratings: Vec<SafetyRating>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
}

#[derive(Serialize, Deserialize)]
struct Prompt {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct RequestPayload {
    prompt: Prompt,
}

pub struct APIRequestClient {
    api_url: String,
    client: Client,
    api_key: String,
}

const API_URL: &'static str = "https://generativelanguage.googleapis.com";

impl APIRequestClient {
    pub fn new(api_key: &str) -> Self {
        let client = Client::new();
        Self {
            api_url: API_URL.to_string(),
            client,
            api_key: api_key.to_string(),
        }
    }

    async fn post_data<T: Serialize>(&self, endpoint: &str, data: &T) -> Result<String, Error> {
        let body = json!(data).to_string();
        let response = self
            .client
            .post(&format!(
                "{}/{}?key={}&temperature=0",
                self.api_url, endpoint, self.api_key
            ))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        let response_text = response.text().await?;
        Ok(response_text)
    }

    pub async fn send_request(&self, text: &str) -> Result<String, Error> {
        let payload = RequestPayload {
            prompt: Prompt {
                text: text.to_string(),
            },
        };
        self.post_data("v1beta2/models/text-bison-001:generateText", &payload)
            .await
    }
}
