use clap::Parser;
use reqwest::Client;
use std::error::Error;
use rust_forvo_api::{create_forvo_url, download_mp3, strip_acute, parse_pronunciations, find_highest_score};

/// Struct to represent command-line arguments using clap
#[derive(Parser, Debug)]
#[command(
    author = "Alan Duncan <duncan.alan@me.com>",
    version = "0.1.3",
    about = "Downloads Russian pronunciation files from Forvo",
    long_about = None)]
struct Args {
    /// Word to get the pronunciation for
    #[arg(short, long)]
    word: String,

    /// API key for Forvo
    #[arg(short, long)]
    key: String,

    /// Anki collection name
    #[arg(short, long)]
    collection: String,

    /// Download location
    #[arg(short, long)]
    dlpath: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();
    let stripped_word = strip_acute(&args.word);

    // Create the pronunciation list URL
    let url = create_forvo_url(&args.key, &stripped_word);

    let client = Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        // Parse the JSON response
        let json: serde_json::Value = response.json().await?;
        
        // Parse JSON into pronunciations
        let pronunciations = parse_pronunciations(&json);

        // Find the pronunciation with the highest score
        if let Some(highest_score_pronunciation) = find_highest_score(&pronunciations) {
            let max_pronunciation_url = highest_score_pronunciation.pathmp3.clone();
            
            // Download the MP3 file with the highest score
            let path = download_mp3(&max_pronunciation_url, &args.dlpath, &stripped_word).await?;
            println!("Pronunciation downloaded to: {}", path);
        } else {
            println!("No pronunciation found with a high score.");
        }
    } else {
        println!("Request failed with status: {:?}", response.status());
    }
        
    Ok(())
}

