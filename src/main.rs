// src/main.rs

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use finalproject::data::{load_restaurants, load_reviews, load_users};
use finalproject::recommend::recommend_for;

/// Recommend Indianapolis restaurants for a user.
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Recommend restaurants using collaborative filtering"
)]
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

fn main() -> Result<()> {
    let args = Cli::parse();

    // 1) Load datasets
    let restaurants = load_restaurants(&args.restaurants)?;
    let reviews     = load_reviews(&args.reviews)?;
    let users       = load_users(&args.users)?;

    // 2) Decide which user to target
    let target_id = match args.user_id.clone() {
        Some(id) => id,
        None => reviews
            .first()
            .map(|r| r.user_id.clone())
            .expect("No user_id provided and reviews file is empty"),
    };

    // 3) Retrieve a human-friendly name if available
    let user_name = users
        .iter()
        .find(|u| u.user_id == target_id)
        .and_then(|u| u.name.clone())
        .unwrap_or_else(|| "<unknown>".to_string());

    // 4) Compute top-5 recommendations
    let recs = recommend_for(&target_id, &restaurants, &reviews, &users);

    // 5) Print
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
