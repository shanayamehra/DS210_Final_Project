# Written Report

## Introduction and Dataset Selection

This project explores how to recommend Indianapolis restaurants to a user by combining their own past ratings with those of similar users using collaborative‐filtering graph techniques. I selected a cleaned subset of the Yelp Open Dataset—specifically all Boston/Indianapolis restaurants, reviews, and users—because it captures real-world dining preferences and friend‐influenced decisions. The cleaned dataset contains roughly 1,200 restaurants, 25,000 reviews, and 5,000 users, which is large enough to demonstrate nontrivial graph computations (user–restaurant bipartite and user–user social graphs) while remaining small enough to ship in GitHub and run quickly. Before analysis, I verified that each JSONL file parsed correctly, that restaurant entries included “Indianapolis” (case‐insensitive) and “restaurant” in their categories, and that review and user files referenced only those business or user IDs.

## Methodology

The analysis proceeded in these high‐level stages:

1. **Data Cleaning (Python):** Wrote three simple scripts  
   - `clean_restaurants.py` filters Yelp’s `business.json` for Indianapolis restaurants.  
   - `clean_reviews.py` keeps only reviews whose `business_id` matches a cleaned restaurant.  
   - `clean_users.py` retains only users who wrote one of those reviews.  

2. **Data Loading (Rust):** Implemented `load_jsonl<T>` in `data.rs`  
   - Buffered line‐by‐line JSON parsing with `serde_json`.  
   - Exposed `load_restaurants`, `load_reviews`, `load_users`.

3. **Graph Construction & Recommendation (`recommend.rs`):**  
   - Built a map `user_id → Vec<(business_id, stars)>` from reviews.  
   - For a given target user, computed dot‐product similarities with every other user over co‐rated restaurants.  
   - Accumulated weighted sums for each restaurant the target user hasn’t rated.  
   - Normalized by total similarity weight to produce a score.  
   - Sorted descending by score, breaking ties lexicographically by `business_id` for deterministic output.  
   - Returned the top five recommendations.

4. **CLI & Output (`main.rs`):**  
   - Used Clap to parse four flags (`--restaurants`, `--reviews`, `--users`, `--user-id`) that default to our cleaned sample files and the first reviewer if `--user-id` is omitted.  
   - Printed a header and the top‐5 list, each line showing restaurant name, business ID, and score to three decimal places.

## Dataset Preparation

- **Directory:** All raw Yelp JSONL files live in `~/Downloads/`.  
- **Cleaning scripts:** Run in Python to produce:
  - `~/Downloads/cleaned_indianapolis_restaurants.json`  
  - `~/Downloads/cleaned_indianapolis_reviews.json`  
  - `~/Downloads/cleaned_indianapolis_users.json`  
- **Verification:** Each script prints the total records retained and the first five entries for manual inspection.

## Project Structure

finalproject/
├── Cargo.toml
├── README.md
├── src/
│ ├── data.rs # JSONL loader & record types
│ ├── recommend.rs # CF algorithm, weighting & tie-breaker sort
│ ├── main.rs # CLI parsing & result printing
│ └── lib.rs # Module exports
├── src/dataset/ # Cleaned JSONL subsets for demo
│ ├── cleaned_indianapolis_restaurants.json
│ ├── cleaned_indianapolis_reviews.json
│ └── cleaned_indianapolis_users.json
├── tests/
│ └── mod.rs # Unit tests for recommend_for
└── output/
└── output.png # Sample program output screenshot


## Analytical Steps

### Similarity Computation

- **Objective:** Quantify how “close” each user is to the target user based on shared ratings.  
- **Method:** Dot‐product of stars on co‐rated restaurants.  
- **Outcome:** Users with higher overlap and rating agreement yield larger similarity scores.

### Prediction Scoring

- **Objective:** Predict the target’s rating for unseen restaurants.  
- **Method:** Weighted sum of other users’ star ratings, divided by total similarity weight.  
- **Tie-Breaker:** Restaurants with identical normalized scores are ordered by lexicographic `business_id`.

### Top-N Selection

- **Objective:** Provide actionable recommendations.  
- **Method:** Sort predictions descending by score, take top 5.

## Testing and Validation

- **Unit tests** in `tests/mod.rs`:  
  1. **No similar users** → ensures an empty recommendation list when only the target user exists.  
  2. **Simple two‐user case** → verifies correct score computation and business ID selection.  
- **Test results:**

  ```text
  $ cargo test
  running 2 tests
  test no_recommend_when_no_similar_users ... ok
  test recommend_simple_case               ... ok

  test result: ok. 2 passed; 0 failed

Conclusion
By combining straightforward data cleaning in Python with efficient JSONL loading and a dot‐product collaborative‐filter in Rust, this project demonstrates how social influence (via similar users) can drive practical restaurant recommendations. The modular architecture, deterministic sorting, and comprehensive tests ensure correctness, reproducibility, and clarity—fulfilling both educational objectives and real‐world utility.