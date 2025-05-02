// src/tests/mod.rs

use crate::data::Review;
use crate::recommend::recommend_for;

/// Tiny review set for unit tests:
/// A→X(4), B→X(5), B→Y(3), C→Y(4)
fn sample_reviews() -> Vec<Review> {
    vec![
        Review { user_id: "A".into(), business_id: "X".into(), stars: 4.0 },
        Review { user_id: "B".into(), business_id: "X".into(), stars: 5.0 },
        Review { user_id: "B".into(), business_id: "Y".into(), stars: 3.0 },
        Review { user_id: "C".into(), business_id: "Y".into(), stars: 4.0 },
    ]
}

#[test]
fn recommend_simple_case() {
    let revs = sample_reviews();
    let recs = recommend_for("A", &revs, 2);

    // Jaccard(A, B) = 1/2 = 0.5, Jaccard(A, C) = 0
    // Only B contributes to Y: sum = 0.5*3.0 = 1.5, weight = 0.5, score = 1.5/0.5 = 3.0
    assert_eq!(recs.len(), 1);
    assert_eq!(recs[0].business_id, "Y");
    assert!((recs[0].score - 3.0).abs() < 1e-6);
}

#[test]
fn no_recommend_when_no_similar_users() {
    let revs = vec![
        Review { user_id: "A".into(), business_id: "X".into(), stars: 5.0 },
    ];
    let recs = recommend_for("A", &revs, 5);
    assert!(recs.is_empty());
}
