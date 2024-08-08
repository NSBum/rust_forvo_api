# Rust Forvo API Downloader

The Rust Forvo API Downloader is a command-line application that downloads Russian pronunciation files from Forvo. It requires a paid Forvo API key to access the pronunciation data.

## Features

- **Download Pronunciations:** Fetch and download MP3 pronunciation files for Russian words from Forvo.
- **Score Calculation:** Determine the most relevant pronunciation based on user votes and special user bonuses.

## Prerequisites

- A **Forvo API key** is required to access the pronunciation data. You can obtain this from the [Forvo API website](https://api.forvo.com/).

## Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/rust_forvo_api.git
   cd rust_forvo_api
   ```

2. **Build the Project:**

   Ensure you have Rust and Cargo installed. Then, run:

   ```bash
   cargo build --release
   ```

## Usage

To run the application, use the following command:

```bash
./target/release/rust_forvo_api --word "example_word" --key "your_api_key" --dlpath "downloads"
```

### Parameters

- `--word`: The word for which you want to download the pronunciation (in Russian).
- `--key`: Your Forvo API key.
- `--dlpath`: The directory where the MP3 file will be saved.

### Example

Suppose you want to download the pronunciation for the Russian word "собака" (dog). You would run:

```bash
./target/release/rust_forvo_api --word "собака" --key "your_api_key" --dlpath "downloads"
```

This command will download the MP3 file for the word "собака" and save it in the `downloads` directory. The application will choose the pronunciation with the highest score based on user votes and bonuses for specific users.

## How It Works

1. **Strip Accents:** The application removes syllabic stress marks from the input word using Unicode normalization.
2. **API Request:** Constructs a request URL for the Forvo API and retrieves pronunciation data.
3. **Parse JSON:** Parses the JSON response to extract pronunciation information.
4. **Calculate Scores:** Calculates scores for each pronunciation based on the number of positive votes and special user bonuses.
5. **Download MP3:** Downloads the MP3 file for the pronunciation with the highest score.

## Testing

To run the tests, use:

```bash
cargo test
```

## Dependencies

- **tokio:** Asynchronous runtime for Rust.
- **serde:** Serialization and deserialization library.
- **serde_json:** JSON parsing for Rust.
- **regex:** Regular expressions for text manipulation.
- **reqwest:** HTTP client for Rust.
- **clap:** Command-line argument parsing.
- **unicode-normalization:** Unicode normalization for string manipulation.
- **futures:** Asynchronous programming utilities.

## Caveats

Just a few caveats about this project:

- I've tested this only on macOS. YMMV if you are on another platform
- I only have need of Russian language pronunciations which is what this downloads. If you need other languages, feel free to modify accordingly.

## Roadmap

- move downloaded pronunciation file into Anki collection
- save the collection setting
- allow for other languages

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you'd like to contribute to this project.

## Contact

For questions or support, please contact [duncan.alan@me.com](mailto:duncan.alan@me.com).

### Key Changes

- **Focused Description:** The README now centers around the application's main function: downloading Russian pronunciation files.
- **Simplified Features:** Removed extra features to focus on the core functionality.
- **Example Usage:** Added a clear example showing how to run the application, including the parameters required.
- **How It Works:** Briefly describes the application's workflow to help users understand its operation.
- **Prerequisites:** Highlights the need for a Forvo API key and provides a link to obtain one.

This README should provide users with a clear understanding of how to use your application effectively. Let me know if there's anything else you'd like to add or modify!
