
// tests/integration.rs
use assert_cmd::Command;

#[test]
fn smoke_test_cli() {
let mut cmd = Command::cargo_bin("finalproject").unwrap();
cmd.arg("--restaurants")
.arg("tests/data/sample_restaurants.json")
.arg("--reviews")
.arg("tests/data/sample_reviews.json")
.arg("--users")
.arg("tests/data/sample_users.json")
.arg("--user-id")
.arg("test_user")
.assert()
.success();
}
