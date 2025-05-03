// tests/mod.rs

use finalproject::data::{Restaurant, Review, User};
use finalproject::recommend::recommend_for;

#[test]
fn no_recommend_when_no_similar_users() {
    // One restaurant, one review, one user → no one else to compare against
    let restaurants = vec![
        Restaurant {
            business_id: "b1".into(),
            name:        "A".into(),
        },
    ];
    let reviews = vec![
        Review {
            user_id:     "u1".into(),
            business_id: "b1".into(),
            stars:       5.0,
        },
    ];
    let users = vec![
        User {
            user_id: "u1".into(),
            name:    None,
        },
    ];

    let recs = recommend_for("u1", &restaurants, &reviews, &users);
    assert!(recs.is_empty());
}

#[test]
fn recommend_simple_case() {
    // u1 rated b1; u2 rated b1 & b2.
    // Similarity(u1,u2) = 5*5 = 25
    // Predicted score for b2 = (25 * 3.0) / 25 = 3.0
    let restaurants = vec![
        Restaurant {
            business_id: "b1".into(),
            name:        "X".into(),
        },
        Restaurant {
            business_id: "b2".into(),
            name:        "Y".into(),
        },
    ];
    let reviews = vec![
        Review {
            user_id:     "u1".into(),
            business_id: "b1".into(),
            stars:       5.0,
        },
        Review {
            user_id:     "u2".into(),
            business_id: "b1".into(),
            stars:       5.0,
        },
        Review {
            user_id:     "u2".into(),
            business_id: "b2".into(),
            stars:       3.0,
        },
    ];
    let users = vec![
        User {
            user_id: "u1".into(),
            name:    None,
        },
        User {
            user_id: "u2".into(),
            name:    None,
        },
    ];

    let recs = recommend_for("u1", &restaurants, &reviews, &users);
    assert_eq!(recs.len(), 1);
    // Score should be 3.0 as per the calculation above
    assert!((recs[0].score - 3.0).abs() < 1e-6);
    // And the recommended business should be b2
    assert_eq!(recs[0].business_id, "b2");
}
