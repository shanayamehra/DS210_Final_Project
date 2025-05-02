// src/recommend.rs

use crate::data::Review;
use std::collections::{HashMap, HashSet};

/// A single recommendation result.
#[derive(Debug)]
pub struct Recommendation {
    pub business_id: String,
    pub score:       f32,
}

/// Build a map: user_id → set of business_ids they’ve reviewed.
fn build_user_map(reviews: &[Review]) -> HashMap<&str, HashSet<&str>> {
    let mut map = HashMap::new();
    for r in reviews {
        map.entry(r.user_id.as_str())
            .or_insert_with(HashSet::new)
            .insert(r.business_id.as_str());
    }
    map
}

/// Compute Jaccard similarity between two users’ review‐sets.
fn jaccard(
    map: &HashMap<&str, HashSet<&str>>,
    u1: &str,
    u2: &str,
) -> f32 {
    let s1 = &map[u1];
    let s2 = &map[u2];
    let inter = s1.intersection(s2).count() as f32;
    let uni   = s1.union(s2).count() as f32;
    if uni > 0.0 { inter / uni } else { 0.0 }
}

/// Recommend top‐N businesses for `user_id` using weighted CF.
pub fn recommend_for(
    user_id: &str,
    reviews: &[Review],
    top_n: usize,
) -> Vec<Recommendation> {
    // 1) Build user→business sets
    let user_map = build_user_map(reviews);
    let target_set = user_map
        .get(user_id)
        .cloned()
        .unwrap_or_default();

    // 2) Build rating lookup
    let mut rating_map = HashMap::new();
    for r in reviews {
        rating_map.insert(
            (r.user_id.as_str(), r.business_id.as_str()),
            r.stars,
        );
    }

    // 3) Accumulate weighted scores
    let mut sum:     HashMap<&str, f32> = HashMap::new();
    let mut weights: HashMap<&str, f32> = HashMap::new();

    for (&other, bizs) in &user_map {
        if other == user_id { continue; }
        let sim = jaccard(&user_map, user_id, other);
        if sim <= 0.0 { continue; }
        for &biz in bizs {
            if target_set.contains(biz) { continue; }
            let rating = *rating_map.get(&(other, biz)).unwrap_or(&0.0);
            *sum.entry(biz).or_default()     += sim * rating;
            *weights.entry(biz).or_default() += sim;
        }
    }

    // 4) Normalize and sort
    let mut recs: Vec<Recommendation> = sum.into_iter()
        .filter_map(|(biz, total)| {
            let w = weights.get(biz)?;
            if *w > 0.0 {
                Some(Recommendation {
                    business_id: biz.to_string(),
                    score:        total / *w,
                })
            } else {
                None
            }
        })
        .collect();

    recs.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    recs.truncate(top_n);
    recs
}
