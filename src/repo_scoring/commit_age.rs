use async_trait::async_trait;
use chrono;
use octocrab::models::{repos::RepoCommit, Repository};

use crate::{github, repo_scoring::ScoringProvider};

pub struct CommitAgeScore<'a> {
    pub owner: String,
    pub name: String,
    pub repo: &'a Repository,
}

#[async_trait]
impl<'a> ScoringProvider<'a> for CommitAgeScore<'a> {
    fn from_repository<'r: 'a>(&self, repo: &'r Repository) -> Self {
        CommitAgeScore {
            owner: match &repo.owner {
                Some(owner) => owner.login.clone(),
                None => "".to_string(),
            },
            name: repo.name.clone(),
            repo,
        }
    }

    ///This function returns the score for the repository.
    async fn score(&self) -> u32 {
        match get_latest_commit(self).await {
            Some(commit) => {
                let now = chrono::Utc::now();
                let author_node = commit.commit.author;

                // We really do need that commit date, and if it's missing
                // then that does indicate that the response from GitHub
                // is not what we expect.

                if let None = author_node {
                    panic!("Somehow we have a commit, but the node where the commit date is stashed is missing.");
                }
                let commit_date_node = author_node.unwrap().date;
                if let None = commit_date_node {
                    panic!("Somehow we have a commit, but the commit date is missing.");
                }
                let commit_date = commit_date_node.unwrap();
                // Let's get the age of the commit in days.
                let commit_age = now.signed_duration_since(commit_date).num_days();

                // If the commit is less than 30 days old, then we'll give it a score of zero
                if commit_age <= 30 {
                    return 0;
                }

                // Now let's get the age of the repository
                if let None = self.repo.created_at {
                    panic!("Somehow we have a repository, but the created_at date is missing.");
                }
                let repo_age = now.signed_duration_since(self.repo.created_at.unwrap()).num_days();
                let mid_point = repo_age / 2;
                if commit_age <= mid_point {
                    return 1;
                }

                // If we're still here, then the commit is older than the mid-point.
                2
            }
            None => 3,
        }
    }
}

pub async fn get_latest_commit<'a>(input: &'a CommitAgeScore<'_>) -> Option<RepoCommit> {
    let gh = github::octocrab_handle();

    let commits = match gh
        .repos(&input.owner, &input.name)
        .list_commits()
        .per_page(1)
        .send()
        .await
    {
        Ok(commits) => commits,
        Err(e) => panic!("Failed to fetch commits: {:?}", e),
    };

    if commits.items.is_empty() {
        return None;
    }

    Some(commits.items[0].clone())
}
