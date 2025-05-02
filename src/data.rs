// src/data.rs

use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// A single restaurant record (ID + name).
#[derive(Debug, Deserialize)]
pub struct Restaurant {
    pub business_id: String,
    pub name:        String,
}

/// A single review record (user, business, stars).
#[derive(Debug, Deserialize)]
pub struct Review {
    pub user_id:     String,
    pub business_id: String,
    pub stars:       f32,
}

/// A single user record (ID + name).
#[derive(Debug, Deserialize)]
pub struct User {
    pub user_id: String,
    pub name:    String,
}

/// Generic JSONL loader.
fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &str) -> Result<Vec<T>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();
    for line in reader.lines() {
        let rec: T = serde_json::from_str(&line?)?;
        out.push(rec);
    }
    Ok(out)
}

/// Load restaurants.
pub fn load_restaurants(path: &str) -> Result<Vec<Restaurant>> {
    load_jsonl(path)
}

/// Load reviews.
pub fn load_reviews(path: &str) -> Result<Vec<Review>> {
    load_jsonl(path)
}

/// Load users.
pub fn load_users(path: &str) -> Result<Vec<User>> {
    load_jsonl(path)
}
