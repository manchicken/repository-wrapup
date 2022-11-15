use octocrab::models;
use std::env;

/// This function returns a live Octocrab instance.
pub fn octocrab_handle() -> octocrab::Octocrab {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");

    match octocrab::Octocrab::builder().personal_token(token).build() {
        Ok(octocrab) => octocrab,
        Err(e) => panic!("Failed to create Octocrab instance: {:?}", e),
    }
}

pub async fn get_single_repo(
    gh: &octocrab::Octocrab,
    owner: &str,
    name: &str,
) -> octocrab::Result<models::Repository> {
    match gh.repos(owner, name).get().await {
        Ok(repo) => Ok(repo),
        Err(e) => panic!("Failed to fetch repository: {:?}", e),
    }
}
