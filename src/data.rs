// src/data.rs
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Deserialize, Clone)]
pub struct Restaurant {
    pub business_id: String,
    pub name: String,
    pub city: String,
    pub state: String,
    pub stars: f32,
    pub review_count: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Review {
    pub user_id: String,
    pub business_id: String,
    pub stars: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub user_id: String,
    pub name: Option<String>,
    pub review_count: usize,
    pub average_stars: f32,
}

/// Generic loader for any JSON-lines file
pub fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &str) -> Vec<T> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|json| serde_json::from_str(&json).ok())
        .collect()
}

/// Specialized loaders
pub fn load_restaurants(path: &str) -> Vec<Restaurant> {
    load_jsonl(path)
}

pub fn load_reviews(path: &str) -> Vec<Review> {
    load_jsonl(path)
}

pub fn load_users(path: &str) -> Vec<User> {
    load_jsonl(path)
}