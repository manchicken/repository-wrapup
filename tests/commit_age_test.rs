use repository_wrapup::{github, repo_scoring::{commit_age, ScoringProvider}};

#[tokio::test]
async fn test_get_commit_age() {
    let repo = github::get_single_repo(
        &github::octocrab_handle(),
        "manchicken",
        "manchicken",
    )
    .await.unwrap();
    let scorer = commit_age::CommitAgeScore {
        owner: "manchicken".to_string(),
        name: "manchicken".to_string(),
        repo: &repo,
    };
    let result = commit_age::get_latest_commit(&scorer).await;
    assert_eq!(result.is_some(), true);

    let score_output = scorer.score().await;
    assert!(score_output <= 1);
}
