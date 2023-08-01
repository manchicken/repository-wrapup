use chrono;
use octocrab::Octocrab;

use crate::github::get_latest_commit;
use crate::repo_scoring::{
  Repo, RepoScoreValue, HIGH_SCORE, LOW_SCORE, MAX_SCORE, MEDIUM_SCORE, MIN_SCORE,
};

// 180 days
const FRESH_DAYS: i64 = 180;
// Three years
const ROTTEN_DAYS: i64 = 365 * 3;

/// This function returns the score for the repository.
pub async fn commit_age_calculate_score(gh: &Octocrab, input: &Repo) -> RepoScoreValue {
  match get_latest_commit(gh, &input.owner, &input.name).await {
    Some(commit) => {
      let now = chrono::Utc::now();
      let author_node = commit.commit.author;

      // We really do need that commit date, and if it's missing
      // then that does indicate that the response from GitHub
      // is not what we expect.

      if author_node.is_none() {
        panic!(
          "Somehow we have a commit, but the node where the commit date is stashed is missing."
        );
      }
      let commit_date_node = author_node.unwrap().date;
      if commit_date_node.is_none() {
        panic!("Somehow we have a commit, but the commit date is missing.");
      }
      let commit_date = commit_date_node.unwrap();
      // Let's get the age of the commit in days.
      let commit_age = now.signed_duration_since(commit_date).num_days();

      // If the commit is within FRESH_DAYS, then we'll give it a minimum score
      if commit_age <= FRESH_DAYS {
        return MIN_SCORE;
      }
      // If the commit is older than ROTTEN_DAYS, then we'll give it a high score
      else if commit_age > ROTTEN_DAYS {
        return HIGH_SCORE;
      }

      // Now let's get the age of the repository
      if input.repo.created_at.is_none() {
        panic!("Somehow we have a repository, but the created_at date is missing.");
      }
      let repo_age = now
        .signed_duration_since(input.repo.created_at.unwrap())
        .num_days();
      let mid_point = repo_age / 2;
      if commit_age <= mid_point {
        return LOW_SCORE;
      }

      // If we're still here, then the commit is older than the mid-point.
      MEDIUM_SCORE
    }
    None => MAX_SCORE,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::github::{get_single_repo, octocrab_handle};

  const TEST_REPO_OWNER: &str = "manchicken";
  const TEST_REPO_NAME: &str = "testing-repository-wrapup";

  #[tokio::test]
  async fn test_commit_age_calculate_score() {
    let gh = octocrab_handle();
    let repo = Repo {
      name: String::from(TEST_REPO_NAME),
      owner: String::from(TEST_REPO_OWNER),
      repo: get_single_repo(&gh, TEST_REPO_OWNER, TEST_REPO_NAME)
        .await
        .unwrap(),
    };

    let result = commit_age_calculate_score(&gh, &repo).await;
    assert!(
      (MIN_SCORE..=MAX_SCORE).contains(&result),
      "The score should be between the MIN and MAX scores."
    );
  }
}
