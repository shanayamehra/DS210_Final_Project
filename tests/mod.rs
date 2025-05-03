// tests/mod.rs
//! Unit tests for the recommendation system. Verifies behavior on edge cases
//! and simple scenarios to ensure algorithm correctness.

use finalproject::data::{Restaurant, Review, User};
use finalproject::recommend::recommend_for;

/// Test that when only one user exists, no recommendations are produced
/// because there are no similar users to base predictions on.
#[test]
fn no_recommend_when_no_similar_users() {
    // Setup: one restaurant, one review, one user
    let restaurants = vec![
        Restaurant { business_id: "b1".into(), name: "A".into() },
    ];
    let reviews = vec![
        Review { user_id: "u1".into(), business_id: "b1".into(), stars: 5.0 },
    ];
    let users = vec![
        User { user_id: "u1".into(), name: None },
    ];

    // Execute: run recommendation for the only user
    let recs = recommend_for("u1", &restaurants, &reviews, &users);

    // Verify: no recommendations should be returned
    assert!(recs.is_empty(), "Expected no recommendations when no similar users");
}

/// Test a simple two-user scenario with one overlapping review:
/// u1 and u2 both rate b1, and u2 also rates b2. We expect b2 recommended to u1.
#[test]
fn recommend_simple_case() {
    // Setup: two restaurants, three reviews, two users
    let restaurants = vec![
        Restaurant { business_id: "b1".into(), name: "X".into() },
        Restaurant { business_id: "b2".into(), name: "Y".into() },
    ];
    let reviews = vec![
        Review { user_id: "u1".into(), business_id: "b1".into(), stars: 5.0 },
        Review { user_id: "u2".into(), business_id: "b1".into(), stars: 5.0 },
        Review { user_id: "u2".into(), business_id: "b2".into(), stars: 3.0 },
    ];
    let users = vec![
        User { user_id: "u1".into(), name: None },
        User { user_id: "u2".into(), name: None },
    ];

    // Execute: get recommendations for u1
    let recs = recommend_for("u1", &restaurants, &reviews, &users);

    // Verify: exactly one recommendation (b2) with correct score ≈ 3.0
    assert_eq!(recs.len(), 1, "Expected one recommendation for simple case");

    let rec = &recs[0];
    assert_eq!(rec.business_id, "b2", "Expected recommended business ID to be b2");
    assert!((rec.score - 3.0).abs() < 1e-6, "Expected score ≈ 3.0, got {}", rec.score);
}