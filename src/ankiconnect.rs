use reqwest::Client;
use serde_json::json;
use std::error::Error;
use std::path::Path;

pub async fn store_mp3_in_collection(mp3_path: &Path) -> Result<serde_json::Value, Box<dyn Error>> {
    let client = Client::new();
    let params = json!({
        "filename": mp3_path.file_name().unwrap().to_str().unwrap(),
        "path": mp3_path.to_str().unwrap(),
    });

    invoke("storeMediaFile", params, &client).await
}

async fn invoke(action: &str, params: serde_json::Value, client: &Client) -> Result<serde_json::Value, Box<dyn Error>> {
    let request_json = json!({
        "action": action,
        "params": params,
        "version": 6,
    });

    let response = client.post("http://localhost:8765")
        .json(&request_json)
        .send()
        .await? // Await the response
        .json::<serde_json::Value>()
        .await?; // Await the conversion to JSON

    if let Some(error) = response.get("error").and_then(|e| e.as_str()) {
        if !error.is_empty() {
            return Err(format!("Error from AnkiConnect: {}", error).into());
        }
    }

    Ok(response["result"].clone())
}

