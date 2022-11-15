use repository_wrapup::{github, repo_scoring::{commit_age, ScoringProvider}};

#[tokio::test]
async fn test_get_commit_age() {
    let scorer = commit_age::CommitAgeScore {
        owner: "manchicken".to_string(),
        name: "manchicken".to_string(),
    };
    let result = commit_age::get_latest_commit(&scorer).await;

    println!("Result: {:#?}", result);
    assert_eq!(result.is_some(), true);
    println!("Score: {:?}", scorer.score().await);
}
