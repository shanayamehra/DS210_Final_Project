// src/main.rs

use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;

use finalproject::{
    data::{load_restaurants, load_reviews, load_users},
    recommend::recommend_for,
};

/// Yelp‐based recommender for Indianapolis restaurants.
#[derive(Parser, Debug)]
#[command(author, version, about = "Yelp-based restaurant recommender")]
struct Args {
    /// Path to cleaned restaurants JSONL
    #[arg(long, default_value = "cleaned_indianapolis_restaurants.json")]
    restaurants: String,

    /// Path to cleaned reviews JSONL
    #[arg(long, default_value = "cleaned_indianapolis_reviews.json")]
    reviews: String,

    /// Path to cleaned users JSONL
    #[arg(long, default_value = "cleaned_indianapolis_users.json")]
    users: String,

    /// User ID to recommend for (optional; defaults to first reviewer)
    #[arg(long)]
    user_id: Option<String>,

    /// Number of recommendations to return
    #[arg(long, default_value_t = 5)]
    top_n: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Load data
    let restaurants = load_restaurants(&args.restaurants)?;
    let mut reviews     = load_reviews(&args.reviews)?;
    let users       = load_users(&args.users)?;

    // 2. Filter out any reviews for businesses we dropped
    let rest_ids: HashMap<_, ()> = restaurants
        .iter()
        .map(|r| (r.business_id.as_str(), ()))
        .collect();
    reviews.retain(|r| rest_ids.contains_key(r.business_id.as_str()));

    // 3. Determine target user
    let target = args.user_id.clone().unwrap_or_else(|| {
        reviews.first()
            .map(|r| r.user_id.clone())
            .unwrap_or_default()
    });

    // 4. Build name lookups
    let rest_names: HashMap<&str, &str> = restaurants
        .iter()
        .map(|r| (r.business_id.as_str(), r.name.as_str()))
        .collect();
    let user_names: HashMap<&str, &str> = users
        .iter()
        .map(|u| (u.user_id.as_str(), u.name.as_str()))
        .collect();

    // 5. Generate recommendations
    let recs = recommend_for(&target, &reviews, args.top_n);

    // 6. Print exactly the Top‐N list
    let user_display = user_names
        .get(target.as_str())
        .unwrap_or(&"<unknown user>");
    println!("\nTop {} recommendations for {} ({})\n",
        args.top_n, user_display, target);

    for (i, r) in recs.iter().enumerate() {
        let name = rest_names
            .get(r.business_id.as_str())
            .unwrap_or(&"<unknown>");
            println!(
                "{:>2}. {:<30} ({}), score = {:.3}",
                i + 1,
                name,
                r.business_id,
                r.score
            );
            
    }

    Ok(())
}

