use repository_wrapup::repo_scoring;
use repository_wrapup::github;

#[tokio::test]
async fn test_simple_load() {
    let repo = github::get_single_repo(
        &github::octocrab_handle(),
        "manchicken",
        "manchicken",
    )
    .await.unwrap();
    let subject = repo_scoring::RepoScoring::new(
        "manchicken".to_string(),
        "manchicken".to_string(),
        &repo,
    );
    assert!(subject.name.eq(&"manchicken".to_string()));
    assert!(subject.repo.name.eq(&"manchicken".to_string()));
}