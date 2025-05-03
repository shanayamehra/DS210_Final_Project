// src/recommend.rs
use crate::data::{Restaurant, Review, User};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Recommendation {
    pub restaurant: Restaurant,
    pub score: f32,
}

/// Compute Jaccard similarity between two sets
fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f32 {
    let inter = a.intersection(b).count() as f32;
    let uni   = a.union(b).count() as f32;
    if uni == 0.0 { 0.0 } else { inter / uni }
}

/// Recommend top-n restaurants for a given user
pub fn recommend_for(
    user_id: &str,
    restaurants: &[Restaurant],
    reviews: &[Review],
    _users: &[User],   // currently unused, but could fetch names etc.
    top_n: usize,
) -> Vec<Recommendation> {
    // 1) build map: user_id → set of business_ids they reviewed
    let mut user_reviews: HashMap<String, HashSet<String>> = HashMap::new();
    for r in reviews {
        user_reviews
            .entry(r.user_id.clone())
            .or_default()
            .insert(r.business_id.clone());
    }

    // 2) get target user's set
    let target = user_reviews.get(user_id).cloned().unwrap_or_default();

    // 3) compute similarity to all other users
    let mut sims: Vec<(String, f32)> = user_reviews
        .iter()
        .filter(|(uid, _)| uid.as_str() != user_id)
        .map(|(uid, set)| (uid.clone(), jaccard(&target, set)))
        .filter(|(_, score)| *score > 0.0)
        .collect();
    sims.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // take top 5 similar users
    let top_users: Vec<_> = sims.iter().take(5).collect();

    // 4) score each restaurant by weighted sum of (similarity × rating)
    let mut rest_score: HashMap<String, f32> = HashMap::new();
    for (other, sim) in top_users {
        for r in reviews.iter().filter(|r| &r.user_id == other) {
            *rest_score.entry(r.business_id.clone()).or_default() += sim * r.stars;
        }
    }

    // 5) assemble Recommendation structs
    let mut recs: Vec<Recommendation> = rest_score.into_iter()
        .filter_map(|(bid, score)| {
            restaurants
                .iter()
                .find(|rest| rest.business_id == bid)
                .map(|rest| Recommendation {
                    restaurant: rest.clone(),
                    score,
                })
        })
        .collect();

    // 6) sort & take top N
    recs.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    recs.into_iter().take(top_n).collect()
}
