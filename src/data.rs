use serde::Deserialize;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Deserialize, Clone)]
pub struct Restaurant {
    pub business_id: String,
    pub name:        String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Review {
    pub user_id:     String,
    pub business_id: String,
    pub stars:       f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub user_id: String,
    pub name:    Option<String>,
}

/// Load a newline-delimited JSONL file of T.
fn load_jsonl<T: for<'de> Deserialize<'de>, P: AsRef<Path>>(path: P) -> io::Result<Vec<T>> {
    let f = File::open(path.as_ref())?;
    let rd = BufReader::new(f);
    let mut v = Vec::new();
    for line_res in rd.lines() {
        let line = line_res?;
        let item: T = serde_json::from_str(&line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        v.push(item);
    }
    Ok(v)
}

pub fn load_restaurants<P: AsRef<Path>>(path: P) -> io::Result<Vec<Restaurant>> {
    load_jsonl(path)
}

pub fn load_reviews<P: AsRef<Path>>(path: P) -> io::Result<Vec<Review>> {
    load_jsonl(path)
}

pub fn load_users<P: AsRef<Path>>(path: P) -> io::Result<Vec<User>> {
    load_jsonl(path)
}
