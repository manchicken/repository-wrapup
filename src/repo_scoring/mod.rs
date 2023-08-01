// use async_trait::async_trait;
use octocrab::models::Repository;
pub mod commit_age;

pub const MIN_SCORE: RepoScoreValue = 0;
pub const LOW_SCORE: RepoScoreValue = 1;
pub const MEDIUM_SCORE: RepoScoreValue = 2;
pub const HIGH_SCORE: RepoScoreValue = 3;
pub const MAX_SCORE: RepoScoreValue = 3;
pub const DEFAULT_SCORE: RepoScoreValue = 0;

#[derive(Debug)]
pub struct Repo {
  pub owner: String,
  pub name: String,
  pub repo: Repository,
}

//
// impl RepoScoreCard {
//   pub fn from_repository(source: &Repository) -> Option<RepoScoreCard> {
//     if let Some(owner) = source.owner.clone() {
//       Some(RepoScoreCard {
//         owner: owner.login,
//         name: source.name.clone(),
//         repo: source.clone(),
//         score: DEFAULT_SCORE,
//       })
//     } else {
//       None
//     }
//   }
// }
//
pub type RepoScoreValue = u32;
//
// #[async_trait]
// pub trait ScoreKeeper {
//   fn name() -> String;
//   async fn calculate_score(gh: &octocrab::Octocrab, score_card: &RepoScoreCard) -> RepoScoreValue;
// }
//
// pub fn get_score_keepers() -> Vec<impl ScoreKeeper> {
//   vec![commit_age::CommitAgeScoreKeeper {}]
// }
