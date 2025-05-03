// src/recommend.rs

use crate::data::{Restaurant, Review, User};
use std::collections::HashMap;

/// A recommendation for a restaurant, with its computed similarity score.
#[derive(Debug, Clone)]
pub struct Recommendation {
    /// The Restaurant record.
    pub restaurant: Restaurant,
    /// The business_id string (duplicated here for convenience).
    pub business_id: String,
    /// The final score (higher means more strongly recommended).
    pub score: f32,
}

/// Compute the top-5 restaurant recommendations for `user_id`
/// by doing a dot-product (cosine-less) collaborative filter.
///
/// # Arguments
///
/// * `user_id`    – the target user’s ID.
/// * `restaurants` – slice of all Restaurant objects.
/// * `reviews`     – slice of all Review objects.
/// * `_users`      – slice of all User objects (unused, but in the API).
///
/// # Returns
///
/// A Vec of at most five `Recommendation`, sorted descending by score,
/// with ties broken lexicographically by `business_id`.
pub fn recommend_for(
    user_id: &str,
    restaurants: &[Restaurant],
    reviews: &[Review],
    _users: &[User],
) -> Vec<Recommendation> {
    // 1) Build a map user_id → Vec<(business_id, stars)>
    let mut user_ratings: HashMap<&str, Vec<(&str, f32)>> = HashMap::new();
    for r in reviews {
        user_ratings
            .entry(&r.user_id)
            .or_default()
            .push((&r.business_id, r.stars));
    }

    // 2) Grab the target user's ratings (or an empty vec if none)
    let empty: Vec<(&str, f32)> = Vec::new();
    let target_ratings = user_ratings.get(user_id).unwrap_or(&empty);

    // 3) Build business_id → Restaurant lookup for cloning later
    let rest_map: HashMap<String, Restaurant> = restaurants
        .iter()
        .cloned()
        .map(|r| (r.business_id.clone(), r))
        .collect();

    // 4) Compute similarity scores (dot product) between target and each other user
    let mut sim_scores: HashMap<&str, f32> = HashMap::new();
    for (&other, their_ratings) in &user_ratings {
        if other == user_id {
            continue;
        }
        let mut dot = 0.0;
        for &(biz_t, stars_t) in target_ratings {
            for &(biz_o, stars_o) in their_ratings {
                if biz_t == biz_o {
                    dot += stars_t * stars_o;
                }
            }
        }
        if dot > 0.0 {
            sim_scores.insert(other, dot);
        }
    }

    // 5) For each unseen business, accumulate weighted sums and weights
    let mut totals: HashMap<&str, f32> = HashMap::new();
    let mut weights: HashMap<&str, f32> = HashMap::new();
    for (&other, &sim) in &sim_scores {
        if let Some(other_rs) = user_ratings.get(other) {
            for &(biz, stars) in other_rs {
                // skip restaurants the target already rated
                if target_ratings.iter().any(|&(tb, _)| tb == biz) {
                    continue;
                }
                *totals.entry(biz).or_default() += sim * stars;
                *weights.entry(biz).or_default() += sim;
            }
        }
    }

    // 6) Turn those totals into Recommendation structs
    let mut recs: Vec<Recommendation> = totals
        .into_iter()
        .filter_map(|(biz, weighted_sum)| {
            weights.get(biz).map(|&w| weighted_sum / w).and_then(|score| {
                rest_map.get(biz).map(|rest| Recommendation {
                    restaurant:  rest.clone(),
                    business_id: biz.to_string(),
                    score,
                })
            })
        })
        .collect();

    // 7) Sort descending by score, tiebreak by business_id, take top 5
    recs.sort_by(|a, b| {
        match b.score.partial_cmp(&a.score).unwrap() {
            std::cmp::Ordering::Equal => a.business_id.cmp(&b.business_id),
            other                     => other,
        }
    });
    recs.into_iter().take(5).collect()
}