//! src/main.rs
//!
//! Command-line interface for the restaurant recommendation system. Parses arguments,
//! loads datasets, runs the CF algorithm, and prints top-N recommendations.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use finalproject::data::{load_restaurants, load_reviews, load_users};
use finalproject::recommend::recommend_for;

/// CLI arguments for `finalproject` binary.
#[derive(Parser, Debug)]
#[command(author, version, about = "Recommend restaurants using collaborative filtering")]
struct Cli {
    /// Path to cleaned restaurants JSONL
    #[arg(
        long,
        default_value = "src/dataset/cleaned_indianapolis_restaurants.json",
        value_name = "FILE"
    )]
    restaurants: PathBuf,

    /// Path to cleaned reviews JSONL
    #[arg(
        long,
        default_value = "src/dataset/cleaned_indianapolis_reviews.json",
        value_name = "FILE"
    )]
    reviews: PathBuf,

    /// Path to cleaned users JSONL
    #[arg(
        long,
        default_value = "src/dataset/cleaned_indianapolis_users.json",
        value_name = "FILE"
    )]
    users: PathBuf,

    /// The user_id to recommend for; if omitted, uses the first reviewer in the reviews file
    #[arg(long)]
    user_id: Option<String>,
}

/// The main entry point: parse args, load data, pick a target user,
/// run recommendation, and print results.
///
/// # Outputs
/// Prints top-N recommendations to stdout in the format:
/// `1. Name (business ID: ID) — score = SSS`
fn main() -> Result<()> {
    // 1) Parse CLI parameters
    let args = Cli::parse();

    // 2) Load datasets from provided paths
    let restaurants = load_restaurants(&args.restaurants)
        .expect("Failed to load restaurants JSONL");
    let reviews = load_reviews(&args.reviews)
        .expect("Failed to load reviews JSONL");
    let users = load_users(&args.users)
        .expect("Failed to load users JSONL");

    // 3) Decide which user to target: explicit flag or first review author
    let target_id = match args.user_id.clone() {
        Some(id) => id,
        None => reviews
            .first()
            .map(|r| r.user_id.clone())
            .expect("No user_id provided and reviews file is empty"),
    };

    // 4) Retrieve a human-friendly name if available, else placeholder
    let user_name = users
        .iter()
        .find(|u| u.user_id == target_id)
        .and_then(|u| u.name.clone())
        .unwrap_or_else(|| "<unknown>".to_string());

    // 5) Compute top-5 recommendations via CF algorithm
    let recs = recommend_for(&target_id, &restaurants, &reviews, &users);

    // 6) Print header and each recommendation line
    println!(
        "Top {} recommendations for {} (user ID: {}):",
        recs.len(),
        user_name,
        target_id
    );
    for (i, rec) in recs.iter().enumerate() {
        println!(
            "{}. {} (business ID: {}) — score = {:.3}",
            i + 1,
            rec.restaurant.name,
            rec.business_id,
            rec.score
        );
    }

    Ok(())
}