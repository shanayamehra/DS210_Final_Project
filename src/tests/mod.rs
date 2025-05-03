// src/tests/mod.rs
use finalproject::recommend::{recommend_for, Recommendation};
use finalproject::data::{Restaurant, Review, User};

#[test]
fn no_recommend_when_no_similar_users() {
    let restaurants = vec![
        Restaurant { business_id: "b1".into(), name: "A".into(), city: "".into(), state: "".into(), stars: 0.0, review_count: 0 },
    ];
    let reviews = vec![
        Review { user_id: "u1".into(), business_id: "b1".into(), stars: 5.0 },
    ];
    let users = vec![
        User { user_id: "u1".into(), name: None, review_count: 1, average_stars: 5.0 },
    ];

    let recs: Vec<Recommendation> = recommend_for("u1", &restaurants, &reviews, &users, 5);
    assert!(recs.is_empty());
}

#[test]
fn recommend_simple_case() {
    let restaurants = vec![
        Restaurant { business_id: "b1".into(), name: "X".into(), city: "".into(), state: "".into(), stars: 0.0, review_count: 0 },
        Restaurant { business_id: "b2".into(), name: "Y".into(), city: "".into(), state: "".into(), stars: 0.0, review_count: 0 },
    ];
    let reviews = vec![
        // u1 rated b1, u2 rated b1 & b2
        Review { user_id: "u1".into(), business_id: "b1".into(), stars: 5.0 },
        Review { user_id: "u2".into(), business_id: "b1".into(), stars: 5.0 },
        Review { user_id: "u2".into(), business_id: "b2".into(), stars: 3.0 },
    ];
    let users = vec![
        User { user_id: "u1".into(), name: None, review_count: 1, average_stars: 5.0 },
        User { user_id: "u2".into(), name: None, review_count: 2, average_stars: 4.0 },
    ];

    let recs = recommend_for("u1", &restaurants, &reviews, &users, 1);
    assert_eq!(recs.len(), 1);
    // Jaccard(u1,u2) = 1/2, weighted score = (1/2)*3 = 1.5
    assert!((recs[0].score - 1.5).abs() < 1e-6);
    assert_eq!(recs[0].restaurant.business_id, "b2");
}
