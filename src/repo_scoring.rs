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

pub const MIN_SCORE: u32 = 0;
pub const LOW_SCORE: u32 = 1;
pub const MEDIUM_SCORE: u32 = 2;
pub const MAX_SCORE: u32 = 3;

#[async_trait]
pub trait ScoringProvider<'a> {
    fn from_repository<'r: 'a>(&self, repo: &'r Repository) -> Self;
    async fn score(&self) -> u32;
}