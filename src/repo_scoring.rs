use async_trait::async_trait;
use octocrab::{models::Repository};
pub mod commit_age;

#[derive(Debug)]
pub struct RepoScoring<'a> {
    pub owner: String,
    pub name: String,
    pub repo: &'a Repository,
}

impl RepoScoring<'_> {
    pub fn new<'a>(owner: String, name: String, repo: &'a Repository) -> RepoScoring {
        RepoScoring {
            owner,
            name,
            repo,
        }
    }
}

#[async_trait]
pub trait ScoringProvider<'a> {
    fn from_repository<'r>(&self, repo: &'r Repository) -> Self;
    async fn score(&self) -> u32;
}