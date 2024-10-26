use std::process;

use anyhow::{Context, Result};
use clap::Parser;
use ollama_rs::{
    self,
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
    Ollama,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Maximum value (exclusive) for random number
    #[arg(default_value = "10")]
    max: Option<u128>,

    /// Minimum value (inclusive) for random number
    min: Option<u128>,
}

async fn get_random_number(min: u128, max: u128) -> Result<u128> {
    // Initialize Ollama client
    let ollama = Ollama::default();

    let prompt = format!(
        "You are a random number generator. Generate a random number between {} and {} \
         (inclusive). Respond with ONLY the number, no other text or explanation.",
        min, max
    );

    // Create generation request with specific options to reduce randomness
    let options = GenerationOptions::default()
        .temperature(0.9)  // High temperature for more randomness
        .top_p(0.9)       // Allow more variety in responses
        .top_k(40); // Consider more tokens for more randomness

    let request = GenerationRequest::new("mistral".to_string(), prompt).options(options);

    let response =
        ollama.generate(request).await.context("Failed to generate response from Ollama")?;

    // Parse the response string into a number
    let num = response
        .response
        .trim()
        .parse::<u128>()
        .context("Failed to parse number from Ollama response")?;

    // Ensure the number is within bounds
    if num < min || num > max {
        anyhow::bail!("Generated number {} is out of bounds [{}, {}]", num, min, max);
    }

    Ok(num)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (min, max) = match (args.min, args.max) {
        (None, None) => (0, 10),
        (None, Some(max)) => (0, max),
        (Some(min), Some(max)) => (min, max),
        (Some(_), None) => {
            eprintln!("Error: If min is specified, max must also be specified");
            process::exit(1);
        },
    };

    if min >= max {
        eprintln!("Error: min must be less than max");
        process::exit(1);
    }

    match get_random_number(min, max).await {
        Ok(num) => println!("{}", num),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        },
    }
}
