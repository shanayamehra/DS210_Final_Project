//! src/data.rs
//! Provides data-loading utilities and core record types for the recommendation system.

use serde::Deserialize;
use std::{fs::File, io::{self, BufRead, BufReader}, path::Path};

/// Represents a restaurant with its Yelp business ID and name.
#[derive(Debug, Deserialize, Clone)]
pub struct Restaurant {
    /// Unique identifier for the restaurant in Yelp.
    pub business_id: String,
    /// Human-readable name of the restaurant.
    pub name:        String,
}

/// Represents a single review connecting a user to a restaurant with a star rating.
#[derive(Debug, Deserialize, Clone)]
pub struct Review {
    /// Unique identifier of the user who wrote the review.
    pub user_id:     String,
    /// Yelp business ID of the reviewed restaurant.
    pub business_id: String,
    /// Star rating given by the user.
    pub stars:       f32,
}

/// Represents a user with an optional display name.
#[derive(Debug, Deserialize, Clone)]
pub struct User {
    /// Unique identifier of the user in Yelp.
    pub user_id: String,
    /// Optional human-readable name of the user.
    pub name:    Option<String>,
}

/// Internal helper: load a newline-delimited JSONL file of deserializable records T.
///
/// # Inputs
/// - `path`: any type reference that can convert to a file path.
///
/// # Outputs
/// A vector of `T` on success or an I/O error if reading/parsing fails.
///
/// # High-Level Logic
/// 1. Open the file at the given path.
/// 2. Wrap it in a buffered reader to read line-by-line.
/// 3. For each line, parse JSON into `T` and collect into a vector.
fn load_jsonl<T: for<'de> Deserialize<'de>, P: AsRef<Path>>(path: P) -> io::Result<Vec<T>> {
    // Open the file for reading
    let f = File::open(path.as_ref())?;
    // Wrap file handle in a buffered reader
    let rd = BufReader::new(f);
    // Accumulator for all parsed records
    let mut v = Vec::new();
    // Iterate over each line
    for line_res in rd.lines() {
        // Propagate I/O errors
        let line = line_res?;
        // Parse JSON text into T, converting JSON errors into I/O errors
        let item: T = serde_json::from_str(&line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        // Store the parsed record
        v.push(item);
    }
    Ok(v)
}

/// Load restaurants from a JSONL file.
///
/// # Inputs
/// - `path`: path to a JSONL file where each line is a `Restaurant`.
///
/// # Outputs
/// Vector of `Restaurant` structs or I/O error.
pub fn load_restaurants<P: AsRef<Path>>(path: P) -> io::Result<Vec<Restaurant>> {
    load_jsonl(path)
}

/// Load reviews from a JSONL file.
///
/// # Inputs
/// - `path`: path to a JSONL file where each line is a `Review`.
///
/// # Outputs
/// Vector of `Review` structs or I/O error.
pub fn load_reviews<P: AsRef<Path>>(path: P) -> io::Result<Vec<Review>> {
    load_jsonl(path)
}

/// Load users from a JSONL file.
///
/// # Inputs
/// - `path`: path to a JSONL file where each line is a `User`.
///
/// # Outputs
/// Vector of `User` structs or I/O error.
pub fn load_users<P: AsRef<Path>>(path: P) -> io::Result<Vec<User>> {
    load_jsonl(path)
}