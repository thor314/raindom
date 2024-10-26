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
    /// First argument: either max (if only one provided) or min (if two provided)
    first: Option<u128>,

    /// Second argument (optional): max value when min is provided
    second: Option<u128>,
}

async fn get_random_number(min: u128, max: u128) -> Result<u128> {
    let ollama = Ollama::default();

    let prompt = format!(
        "You are a random number generator. Generate a random number between {} and {} \
         (inclusive). Respond with ONLY the number, no other text or explanation.",
        min, max
    );

    let options = GenerationOptions::default().temperature(0.9).top_p(0.9).top_k(40);

    let request = GenerationRequest::new("mistral".to_string(), prompt).options(options);

    let response =
        ollama.generate(request).await.context("Failed to generate response from Ollama")?;

    let num = response
        .response
        .trim()
        .parse::<u128>()
        .context("Failed to parse number from Ollama response")?;

    if num < min || num > max {
        anyhow::bail!("Generated number {} is out of bounds [{}, {}]", num, min, max);
    }

    Ok(num)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (min, max) = match (args.first, args.second) {
        (None, None) => (0, 10),              // raindom
        (Some(max), None) => (0, max),        // raindom [max]
        (Some(min), Some(max)) => (min, max), // raindom [min] [max]
        (None, Some(_)) => {
            eprintln!("Invalid argument pattern. Usage:");
            eprintln!("  raindom");
            eprintln!("  raindom [max]");
            eprintln!("  raindom [min] [max]");
            process::exit(1);
        },
    };

    if min >= max {
        eprintln!("Error: min ({}) must be less than max ({})", min, max);
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
