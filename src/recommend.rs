use crate::data::{Restaurant, Review, User};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Recommendation {
    pub restaurant:  Restaurant,
    pub business_id: String,
    pub score:       f32,
}

/// Collaborative-filtering based on dot-product of co-ratings.
pub fn recommend_for(
    user_id: &str,
    restaurants: &[Restaurant],
    reviews:     &[Review],
    _users:      &[User],       // unused but kept for signature
) -> Vec<Recommendation> {
    // 1) Aggregate each user's ratings
    let mut user_ratings: HashMap<&str, Vec<(&str, f32)>> = HashMap::new();
    for r in reviews {
        user_ratings
            .entry(&r.user_id)
            .or_default()
            .push((&r.business_id, r.stars));
    }

    // 2) Target user's ratings
    let empty: Vec<(&str, f32)> = Vec::new();
    let target_ratings = user_ratings.get(user_id).unwrap_or(&empty);

    // 3) Build a lookup for Restaurant clones
    let rest_map: HashMap<String, Restaurant> = restaurants
        .iter()
        .cloned()
        .map(|r| (r.business_id.clone(), r))
        .collect();

    // 4) Compute similarity = dot-product over co-rated items
    let mut sim_scores: HashMap<&str, f32> = HashMap::new();
    for (&other, their_ratings) in &user_ratings {
        if other == user_id { continue; }
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

    // 5) For each unseen business, accumulate weighted sums
    let mut totals:  HashMap<&str, f32> = HashMap::new();
    let mut weights: HashMap<&str, f32> = HashMap::new();

    for (&other, &sim) in &sim_scores {
        if let Some(rts) = user_ratings.get(other) {
            for &(biz, stars) in rts {
                if target_ratings.iter().any(|&(tb, _)| tb == biz) {
                    continue;
                }
                *totals.entry(biz).or_default()  += sim * stars;
                *weights.entry(biz).or_default() += sim;
            }
        }
    }

    // 6) Build Recommendations only when we have a Restaurant
    let mut recs: Vec<Recommendation> = totals
        .into_iter()
        .filter_map(|(biz, sum)| {
            weights.get(biz).and_then(|&w| {
                let score = sum / w;
                rest_map.get(biz).map(|rest| Recommendation {
                    restaurant:  rest.clone(),
                    business_id: biz.to_string(),
                    score,
                })
            })
        })
        .collect();

    // 7) Sort & take top 5
    recs.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    recs.into_iter().take(5).collect()
}
