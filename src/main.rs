use clap::Parser;
use reqwest::Client;
use std::error::Error;
use rust_forvo_api::{create_forvo_url, download_mp3, strip_acute, parse_pronunciations, find_highest_score};
use rust_forvo_api::config::{save_config, load_config};
// Hold off on this feature for now
//use rust_forvo_api::ankiconnect::{store_mp3_in_collection};

/// Struct to represent command-line arguments using clap
#[derive(Parser, Debug)]
#[command(
    author = "Alan Duncan <duncan.alan@me.com>",
    version = env!("CARGO_PKG_VERSION"),
    about = "Downloads Russian pronunciation files from Forvo",
    long_about = None)]
struct Args {
    /// Word to get the pronunciation for
    #[arg(short, long)]
    word: Option<String>,

    /// API key for Forvo
    #[arg(short, long)]
    key: Option<String>,

    /// Anki collection name
    #[arg(short, long)]
    collection: Option<String>,

    /// Download location
    #[arg(short, long)]
    dlpath: Option<String>,

    /// Save the API key for future use
    #[arg(long)]
    keysave: Option<String>,

    /// Save the collection as default collection
    #[arg(long)]
    collectionsave: Option<String>,

    /// Set Anki2 path
    #[arg(long)]
    set_anki2_path: Option<String>,
}   

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Load existing config or Create
    let mut config = load_config().unwrap_or_default();
    // Handle saving of API key if that option was provided
    if let Some(key) = args.keysave {
        config.api_key = Some(key);
        match save_config(&config) {
            Ok(_) => println!("API key saved"),
            Err(e) => {
                eprintln!("Error saving API key - {}",e);
                return Ok(());
            }
        };
        return Ok(());
    }

    // Handle saving the default_collection
    if let Some(collection) = args.collectionsave {
        config.default_collection = Some(collection);
        match save_config(&config) {
            Ok(_) => println!("Default collection saved"),
            Err(e) => {
                eprintln!("Error saving default collection - {}", e);
                return Ok(());
            }
        };
        return Ok(());
    }

    // If user setting the Anki2 path then allow that to be set and exit
    if let Some(anki2_path) = args.set_anki2_path {
        config.anki2_path = Some(anki2_path);
        match save_config(&config) {
            Ok(_) => println!("Anki2 path saved"),
            Err(e) => {
                eprintln!("Error saving Anki2 path - {}", e);
                return Ok(());
            }
        };
        return Ok(());
    }


    // Try to load the Anki2 collection path from config
    // but if config doesn't have it, we have to assume
    // that the user wants to just download the file and not
    // move it into an Anki collection
    let anki2_path = &config.anki2_path;
    match anki2_path {
        Some(path) => println!("Anki2 path is {}", path),
        None => println!("Anki2 path has never been set"),
    }
            

    // Use provided API key or try to load from config
    let api_key = match &args.key {
        Some(key) => key.clone(),
        None => match &config.api_key {
            Some(key) => key.clone(),
            None => {
                eprintln!("API key is required");
                return Ok(());
            }
        },
    };

    // Make sure word and dlpath are provided
    let word = match &args.word {
        Some(word) => word.clone(),
        None => {
            eprintln!("Word is required.");
            return Ok(());
        }
    };
    let dlpath = match &args.dlpath {
        Some(dlpath) => dlpath.clone(),
        None => {
            eprintln!("dlpath must be provided");
            return Ok(());
        }
    };

    let stripped_word = strip_acute(&word);

    // Create the pronunciation list URL
    let url = create_forvo_url(&api_key, &stripped_word);

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
            let path = download_mp3(&max_pronunciation_url, &dlpath, &stripped_word).await?;
            println!("Pronunciation downloaded to: {}", path);

            // move the pronunciation file physically into the Anki2 collection
            // do we have an anki2_path and a collection?
           if let Some(ankipath) = anki2_path {
               // we have an Anki2 path
               // check if we have a collection argument
               let collection_name = match &args.collection {
                   Some(collection_name) => collection_name.clone(),
                   None => {
                       // see if it is stored in config
                       match &config.default_collection {
                           Some(name) => name.clone(),
                           None => {
                               println!("No collection name provided or saved. Will not be stored in Anki");
                               return Ok(());
                           }
                       }
                   }
               };
               // Now store mp3 in the collection
               //let final_collection_path = format!("{}/{}/collection.media/{}.mp3",ankipath, collection_name, word);
           }
        } else {
            println!("No pronunciation found with a high score.");
        }
    } else {
        println!("Request failed with status: {:?}", response.status());
    }
        
    Ok(())
}

