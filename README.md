# Indianapolis Restaurant Recommender  
A Rust-based collaborative-filtering system that suggests top Indianapolis restaurants to a user based on Yelp review data.

## Introduction, Dataset Selection & Motivation  
This project answers the question: *How can we recommend Indianapolis restaurants to a user by leveraging both their own past ratings and those of similar reviewers?* To explore this, we extracted a manageable subset of the Yelp Academic Dataset—approximately 1,200 restaurants in Indianapolis (filtered by city and “restaurant” category), 25,000 corresponding reviews, and the 5,000 users who wrote them. Cleaning in Python yields minimal JSONL files containing only `business_id`, `name`, `user_id`, and `stars`, striking a balance between real-world scale and GitHub-friendly size while enabling nontrivial graph computations.

## Methodology  
1. **Data Cleaning (Python)**  
   - `clean_restaurants.py`: filters `business.json` for Indianapolis restaurants.  
   - `clean_reviews.py`: retains only reviews whose `business_id` matches a cleaned restaurant.  
   - `clean_users.py`: keeps only users who authored those reviews.  
2. **Data Loading (Rust)**  
   - `data.rs`: implements `load_jsonl<T>` with `BufReader` + `serde_json` for generic JSONL parsing, and exposes `load_restaurants`, `load_reviews`, `load_users`.  
3. **Recommendation Algorithm**  
   - `recommend.rs`: builds `user_id → Vec<(business_id, stars)>`; computes dot-product similarities; accumulates weighted sums for unseen restaurants; normalizes scores; applies a deterministic tie-breaker on `business_id`; returns top 5.  
4. **CLI & Output**  
   - `main.rs`: uses Clap with defaults pointing at our cleaned sample files and optional `--user-id`; orchestrates loading, recommendation, and formatted printing of top-N results.

## Project Structure 
```text
finalproject/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src/
│   ├── data.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── recommend.rs
│   └── dataset/
│       ├── data_cleaning.ipynb
│       ├── cleaned_indianapolis_restaurants.json
│       ├── cleaned_indianapolis_reviews.json
│       └── cleaned_indianapolis_users.json
├── tests/
│   └── mod.rs
└── output/
    └── output.png
```

## Usage

After cloning and building, you can simply run `cargo run` to invoke the recommender over the included Indianapolis datasets.  
If you’d like to point at your own JSONL files (or a different user), pass the `--restaurants`, `--reviews`, `--users`, and `--user-id` flags to customize the input.

## Testing

A small suite of unit tests verifies that `recommend_for()` behaves correctly in edge cases (no similar users) and in a simple two-user scenario. Run them via `cargo test`—you should see both tests pass successfully before you ship.

## Conclusion

This project showcases a streamlined pipeline combining Python data cleaning and Rust’s performance and type safety to deliver personalized restaurant recommendations. Modular code and unit tests ensure maintainability and correctness. The deterministic collaborative-filtering algorithm provides reproducible top-5 suggestions with a single cargo run, demonstrating the practical value of simple graph-based methods.