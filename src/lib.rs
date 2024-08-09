pub mod config;

use regex::Regex;
use serde::{Deserialize, Serialize};
use reqwest::Client;

/// Structure to represent a pronunciation
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Pronunciation {
    pub id: i32,
    pub hits: i32,
    pub username: String,
    pub pathmp3: String,
    pub num_positive_votes: i32,
    pub score: i32,
}

impl Pronunciation {
    /// Constructor for `Pronunciation` that calculates the score
    pub fn new(id: i32, hits: i32, username: String, pathmp3: String, num_positive_votes: i32) -> Self {
        let score = num_positive_votes + Self::calculate_score_increment(&username);
        Self {
            id,
            hits,
            username,
            pathmp3,
            num_positive_votes,
            score,
        }
    }

    /// Calculate score increment based on username
    fn calculate_score_increment(username: &str) -> i32 {
        let special_users = [
            "1640max", "Spinster", "szurzuncik", "ae5s", "Shady_arc", "zhivanova", "Selene71",
        ];

        if special_users.contains(&username) {
            2
        } else {
            0
        }
    }
}

/// Strip acute accents from words using regex and unicode normalization
pub fn strip_acute(word: &str) -> String {
    let regex = Regex::new(r"\p{Mn}").unwrap();
    regex.replace_all(word, "").to_string()
}

/// Create Forvo URL for a given API key and word
pub fn create_forvo_url(api_key: &str, word: &str) -> String {
    format!(
        "https://apifree.forvo.com/key/{}/format/json/action/word-pronunciations/word/{}/language/ru",
        api_key, word
    )
}

/// Find the pronunciation with the highest score
pub fn find_highest_score(pronunciations: &[Pronunciation]) -> Option<&Pronunciation> {
    pronunciations.iter().max_by_key(|p| p.score)
}

/// Parse a single item from JSON into a Pronunciation struct
pub fn parse_pronunciation_item(item: &serde_json::Value) -> Pronunciation {
    let id = item["id"].as_i64().unwrap_or(0) as i32;
    let hits = item["hits"].as_i64().unwrap_or(0) as i32;
    let username = item["username"].as_str().unwrap_or("").to_string();
    let pathmp3 = item["pathmp3"].as_str().unwrap_or("").to_string();
    let num_positive_votes = item["num_positive_votes"].as_i64().unwrap_or(0) as i32;

    Pronunciation::new(id, hits, username, pathmp3, num_positive_votes)
}

/// Deserialize JSON to a vector of Pronunciation structs
pub fn parse_pronunciations(json: &serde_json::Value) -> Vec<Pronunciation> {
    json["items"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(parse_pronunciation_item)
        .collect()
}

/// Function to download an MP3 file from a URL to a specified directory
pub async fn download_mp3(url: &str, directory: &str, word: &str) -> Result<String, Box<dyn std::error::Error>> {
    use tokio::fs::create_dir_all;
    use tokio::io::AsyncWriteExt;
    use std::path::Path;

    // Create a reqwest client
    let client = Client::new();

    // Send a GET request
    let mut response = client.get(url).send().await?;

    // Ensure the request was successful
    if !response.status().is_success() {
        return Err(format!("Failed to download file: {:?}", response.status()).into());
    }

    // Get the file name from the URL
    let file_name = format!("{}.mp3", word);

    // Create the full path for the file
    let file_path = Path::new(directory).join(file_name);

    // Create the directory if it doesn't exist
    create_dir_all(directory).await?;

    // Open a file in write-only mode
    let mut file = tokio::fs::File::create(&file_path).await?;

    // Stream the response bytes to the file
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }

    println!("File downloaded successfully to {:?}", file_path);

    Ok(file_path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use crate::{
        create_forvo_url, strip_acute, parse_pronunciation_item,
        Pronunciation, parse_pronunciations,
    };
    use serde_json::json;

    #[test]
    fn test_remove_syllabic_stress() {
        let actual = strip_acute("многоба́йтовый");
        let expected = "многобайтовый";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_forvo_url() {
        let api_key = "test_api_key";
        let word = "собака";
        let expected: &str = "https://apifree.forvo.com/key/test_api_key/format/json/action/word-pronunciations/word/собака/language/ru";
        let actual = create_forvo_url(api_key, word);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_pronunciation_item() {
        let json_item = json!({
            "id": 123,
            "hits": 50,
            "username": "1640max",
            "pathmp3": "http://example.com/pronunciation.mp3",
            "num_positive_votes": 5
        });

        let expected = Pronunciation {
            id: 123,
            hits: 50,
            username: "1640max".to_string(),
            pathmp3: "http://example.com/pronunciation.mp3".to_string(),
            num_positive_votes: 5,
            score: 7, // 5 votes + 2 bonus
        };

        let actual = parse_pronunciation_item(&json_item);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_pronunciations() {
        let json_data = json!({
            "items": [
                {
                    "id": 123,
                    "hits": 50,
                    "username": "1640max",
                    "pathmp3": "http://example.com/pronunciation.mp3",
                    "num_positive_votes": 5
                },
                {
                    "id": 456,
                    "hits": 30,
                    "username": "another_user",
                    "pathmp3": "http://example.com/another_pronunciation.mp3",
                    "num_positive_votes": 3
                }
            ]
        });

        let expected = vec![
            Pronunciation {
                id: 123,
                hits: 50,
                username: "1640max".to_string(),
                pathmp3: "http://example.com/pronunciation.mp3".to_string(),
                num_positive_votes: 5,
                score: 7, // 5 votes + 2 bonus
            },
            Pronunciation {
                id: 456,
                hits: 30,
                username: "another_user".to_string(),
                pathmp3: "http://example.com/another_pronunciation.mp3".to_string(),
                num_positive_votes: 3,
                score: 3, // 3 votes, no bonus
            }
        ];

        let actual = parse_pronunciations(&json_data);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_pronunciation_new() {
        let id = 123;
        let hits = 50;
        let username = "1640max".to_string();
        let pathmp3 = "http://example.com/pronunciation.mp3".to_string();
        let num_positive_votes = 5;

        let pronunciation = Pronunciation::new(id, hits, username, pathmp3, num_positive_votes);

        assert_eq!(pronunciation.score, 7); // 5 votes + 2 bonus
    }

    
}

