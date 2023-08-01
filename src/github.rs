use std::env;

use octocrab::models::{repos::RepoCommit, Repository};

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
) -> octocrab::Result<Repository> {
  match gh.repos(owner, name).get().await {
    Ok(repo) => Ok(repo),
    Err(e) => panic!("Failed to fetch repository: {:?}", e),
  }
}

pub async fn get_latest_commit(
  gh: &octocrab::Octocrab,
  owner: &str,
  name: &str,
) -> Option<RepoCommit> {
  let commits = match gh
    .repos(owner, name)
    .list_commits()
    .per_page(1)
    .send()
    .await
  {
    Ok(commits) => {
      if commits.items.is_empty() {
        None
      } else {
        Some(commits.items[0].clone())
      }
    }
    Err(octocrab::Error::GitHub { source, .. }) => {
      // GOTTA MAKE THIS HANDLE EMPTY REPO EXCEPTIONS!
      let gh_err = source.message;
      if gh_err == "Git Repository is empty." {
        return None;
      }

      panic!(
        "Failed to fetch commits for «{}/{}»: {:?}",
        owner, name, source.errors
      )
    }
    Err(err) => panic!(
      "Failed to fetch commits for «{}/{}»: {:?}",
      owner, name, err
    ),
  };

  commits
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_REPO_OWNER: &str = "manchicken";
  const TEST_REPO_NAME: &str = "testing-repository-wrapup";

  #[tokio::test]
  async fn test_get_single_repo() {
    let gh = octocrab_handle();

    let single_repo = get_single_repo(&gh, TEST_REPO_OWNER, TEST_REPO_NAME).await;
    assert!(single_repo.is_ok());

    let unwrapped_single_repo = single_repo.unwrap();
    assert_eq!(unwrapped_single_repo.name, TEST_REPO_NAME);
    assert_eq!(
      unwrapped_single_repo.full_name,
      Some(format!("{}/{}", TEST_REPO_OWNER, TEST_REPO_NAME))
    );
  }

  #[tokio::test]
  async fn test_get_latest_commit() {
    let gh = octocrab_handle();

    let latest_commit = get_latest_commit(&gh, TEST_REPO_OWNER, TEST_REPO_NAME).await;

    assert!(latest_commit.is_some());
    assert!(!latest_commit.unwrap().sha.is_empty());
  }
}
