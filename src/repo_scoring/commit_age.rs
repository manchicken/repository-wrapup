use async_trait::async_trait;
use octocrab::models::{repos::RepoCommit, Repository};
use chrono;

use crate::{github, repo_scoring::ScoringProvider};

pub struct CommitAgeScore {
    pub owner: String,
    pub name: String,
}

#[async_trait]
impl ScoringProvider<'_> for CommitAgeScore {
    fn from_repository<'r>(&self, repo: &'r Repository) -> Self {
        CommitAgeScore {
            owner: match &repo.owner {
                Some(owner) => owner.login.clone(),
                None => "".to_string(),
            },
            name: repo.name.clone(),
        }
    }

    async fn score(&self) -> u32 {
        let latest_commit = get_latest_commit(self).await;
        match latest_commit {
            Some(commit) => {
                let now = chrono::Utc::now();
                let commit_date = commit.commit.author.unwrap().date;
                let diff = now.signed_duration_since(commit_date.unwrap());
                // TODO: Do this better.
                let days = diff.num_days() - f64::floor(365 as f64 * 1.5 as f64) as i64;
                if days < 0 {
                    0
                } else {
                    days as u32
                }
            }
            None => 0,
        }
    }
}

pub async fn get_latest_commit<'a>(input: &'a CommitAgeScore) -> Option<RepoCommit> {
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
