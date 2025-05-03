// src/main.rs
use clap::Parser;
use finalproject::data::{load_restaurants, load_reviews, load_users};
use finalproject::recommend::recommend_for;

#[derive(Parser)]
#[command(name = "finalproject")]
struct Args {
    /// Path to restaurants JSON-lines
    #[arg(long)]
    restaurants: String,

    /// Path to reviews JSON-lines
    #[arg(long)]
    reviews: String,

    /// Path to users JSON-lines
    #[arg(long)]
    users: String,

    /// User ID to recommend for
    #[arg(long)]
    user_id: String,

    /// Number of top recommendations
    #[arg(long, default_value_t = 5)]
    top_n: usize,
}

fn main() {
    let args = Args::parse();

    let restaurants = load_restaurants(&args.restaurants);
    let reviews     = load_reviews(&args.reviews);
    let users       = load_users(&args.users);

    let recs = recommend_for(
        &args.user_id,
        &restaurants,
        &reviews,
        &users,
        args.top_n,
    );

    println!("Top {} recommendations for {}:", args.top_n, args.user_id);
    for (i, r) in recs.iter().enumerate() {
        println!(
            "{}. {} ({}) — score = {:.3}",
            i + 1,
            r.restaurant.name,
            r.restaurant.business_id,
            r.score
        );
    }
}
