use bitflags::bitflags;
use clap::Parser;
use repository_wrapup::github::{get_latest_commit, octocrab_handle};
use std::collections::HashMap;
// use std::fs::File;

use log::*;

use simplelog::*;

use octocrab::{
  models::{repos::RepoCommit, Author, Repository},
  Octocrab,
};

// 180 days
const FRESH_DAYS: i64 = 180;
// Three years
const ROTTEN_DAYS: i64 = 365 * 3;

#[derive(Parser, Debug)]
#[command(author, version, arg_required_else_help(true))]
struct Opts {
  /// Would you like to include archived repositories?
  #[arg(short = 'a', long, default_value_t = false)]
  archived: bool,

  /// Would you like to enable debug information?
  #[arg(short = 'd', long, default_value_t = false)]
  debug: bool,

  /// Would you like to include forks?
  #[arg(short = 'f', long, default_value_t = false)]
  forks: bool,

  /// Would you like to print to a CSV file?
  #[arg(short = 'c', long)]
  csv_file: Option<String>,

  /// What is the name of the GitHub organization?
  #[arg(short = 'o', long)]
  org: String,
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct AbandonedType: u8 {
    const NOT_ABANDONED = 0b0000;
    const EMPTY_REPO = 0b0001;
    const UNTOUCHED_IN_AGES = 0b0010;
    const MISSING_MAINTAINERS = 0b0100;
    const FRESH_COMMITS = 0b1000;
    const ABANDONED_AND_MISSING_REPO = 0b0110;
    const MISSING_MAINTAINERS_BUT_FRESH_COMMITS = 0b1100;
  }
}

/// Fetch a map of all of the users in the organization, and then produce a map of their usernames to their
/// user objects. This will make it easier to quickly check whether or not a user is a member of the organization.
async fn get_user_map(opts: &Opts, gh: &Octocrab) -> HashMap<String, Author> {
  let orgs_handle = gh.orgs(opts.org.clone());

  let mut to_return: HashMap<String, Author> = HashMap::new();

  let mut page_number = 0u32;

  loop {
    let current_page = match orgs_handle
      .list_members()
      .per_page(100u8)
      .page(page_number)
      .send()
      .await
    {
      Ok(res) => res,
      Err(err) => panic!("Unable to fetch users: {:#?}", err),
    };

    // Stop looking if we're not getting any more items
    if current_page.items.is_empty() {
      debug!("Finished paging through members.");
      break;
    }

    for author in current_page {
      to_return.insert(author.login.clone(), author);
    }

    page_number += 1;
  }

  to_return
}

#[derive(Debug)]
pub struct RepoCommitPair {
  pub repo: Repository,
  pub last_commit: Option<RepoCommit>,
}

fn should_skip_repo(opts: &Opts, repo: &Repository) -> bool {
  // Skip over archived repositories.
  if opts.archived || repo.archived.unwrap_or(false) {
    return true;
  }
  // Skip over forks, if asked to.
  if opts.forks || repo.fork.unwrap_or(false) {
    return true;
  }

  false
}

async fn process_repositories(
  opts: &Opts,
  gh: &Octocrab,
  user_map: &HashMap<String, Author>,
) -> u32 {
  let orgs_handle = gh.orgs(opts.org.clone());

  let mut to_return = 0u32;
  let mut page_number = 0u32;

  'pagination_loop: loop {
    let current_page = match orgs_handle
      .list_repos()
      .per_page(100u8)
      .page(page_number)
      .send()
      .await
    {
      Ok(res) => res,
      Err(err) => panic!("Unable to fetch repositories: {:#?}", err),
    };

    // Stop looking if we're not getting any more items
    if current_page.items.is_empty() {
      debug!("Finished paging through repositories.");
      break 'pagination_loop;
    }

    'inside_page_loop: for repo in current_page {
      if should_skip_repo(opts, &repo) {
        continue 'inside_page_loop;
      }

      let repo_val = RepoCommitPair {
        repo,
        last_commit: None,
      };

      to_return += 1;
      let abandoned_type = get_repository_abandoned_type(&repo_val, user_map, gh).await;
      get_report_line(&repo_val, &abandoned_type).await;
    }

    page_number += 1;
  }

  to_return
}

/// Fetch the latest commit for the repository provided, and then use that information to determine whether
/// or not the repository is abandoned. Return the abandonment status.
async fn get_repository_abandoned_type(
  repo: &RepoCommitPair,
  user_map: &HashMap<String, Author>,
  gh: &Octocrab,
) -> AbandonedType {
  let mut to_return: AbandonedType = AbandonedType::NOT_ABANDONED;
  let name = repo.repo.name.as_ref();
  let owner = repo
    .repo
    .owner
    .as_ref()
    .unwrap_or_else(|| panic!("Somehow this repo has no owner!"))
    .login
    .as_ref();

  let last_commit: RepoCommit = match get_latest_commit(gh, owner, name).await {
    Some(commit) => commit,
    None => {
      debug!("No commits found for repo, it is empty: {}", name);
      return AbandonedType::EMPTY_REPO;
    }
  };

  // OFI: It would be neat if we had the ability to check for whether or not _any_ of the maintainers
  //      are still in the organization.
  if let Some(author_entry) = last_commit.committer.as_ref() {
    let committer_login = &author_entry.login;
    if user_map.get(committer_login).is_none() {
      debug!("Last commit was by a non-member: {}", committer_login);
      to_return |= AbandonedType::MISSING_MAINTAINERS;
    }
  }

  let now = chrono::Utc::now();
  let author_node = last_commit.commit.author;
  if author_node.is_none() {
    panic!("Somehow we have a commit, but the node where the commit date is stashed is missing.");
  }

  let commit_date_node = author_node.unwrap().date;
  if commit_date_node.is_none() {
    panic!("Somehow we have a commit, but the commit date is missing.");
  }
  let commit_date = commit_date_node.unwrap();

  let commit_age = now.signed_duration_since(commit_date).num_days();

  // If the commit is within FRESH_DAYS, then we'll give it a minimum score
  if commit_age <= FRESH_DAYS {
    return to_return | AbandonedType::FRESH_COMMITS;
  }
  // If the commit is older than ROTTEN_DAYS, then the repo is abandoned
  else if commit_age > ROTTEN_DAYS {
    return to_return | AbandonedType::UNTOUCHED_IN_AGES;
  }

  // Now let's get the age of the repository
  if repo.repo.created_at.is_none() {
    panic!("Somehow we have a repository, but the created_at date is missing.");
  }
  let repo_age = now
    .signed_duration_since(repo.repo.created_at.unwrap())
    .num_days();
  let mid_point = repo_age / 2;

  // If the commit is older than the midpoint, then the repo is abandoned
  if commit_age > mid_point {
    to_return |= AbandonedType::UNTOUCHED_IN_AGES;
  }

  to_return
}

/// Generate a report for the repository provided
async fn get_report_line(repo_val: &RepoCommitPair, abandoned_type: &AbandonedType) {
  match *abandoned_type {
    AbandonedType::MISSING_MAINTAINERS_BUT_FRESH_COMMITS => {
      warn!("Repo {} does have fresh commits, but they were made by someone who is not a member of this org.", repo_val.repo.name);
    }
    AbandonedType::ABANDONED_AND_MISSING_REPO => {
      warn!("Repo {} hasn't been touched in more than {} days, and the last person to commit to it is not presently in the org.", repo_val.repo.name, ROTTEN_DAYS);
    }
    AbandonedType::MISSING_MAINTAINERS => {
      warn!(
        "Repo {}'s last commit was not made by a member of this org.",
        repo_val.repo.name
      );
    }
    AbandonedType::EMPTY_REPO => {
      warn!("Repo {} is empty.", repo_val.repo.name);
    }
    AbandonedType::UNTOUCHED_IN_AGES => {
      warn!(
        "Repo {} hasn't been touched in more than {} days.",
        repo_val.repo.name, ROTTEN_DAYS
      );
    }
    AbandonedType::NOT_ABANDONED | AbandonedType::FRESH_COMMITS => {
      info!("Repo is not abandoned: {}", repo_val.repo.name);
    }
    _ => {
      panic!("Unhandled case for repo: {}", repo_val.repo.name);
    }
  }
}

#[tokio::main]
async fn main() {
  let opts = Opts::parse();

  let log_level = if opts.debug {
    LevelFilter::Debug
  } else {
    LevelFilter::Info
  };

  if let Err(e) = TermLogger::init(
    log_level,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  ) {
    panic!("Failed to initialize logger: {:?}", e);
  }

  debug!("DEBUG ENABLED");

  /*
  let csv_fh = if opts.csv_file.is_some() {
    let fname = opts.csv_file.as_ref().unwrap().clone();
    match File::create(&fname) {
      Ok(fh) => Some(fh),
      Err(e) => panic!("Error creating CSV file «{}»: {:?}", fname, e),
    }
  } else {
    None
  };
  */

  let gh = octocrab_handle();

  let user_map: HashMap<String, Author> = get_user_map(&opts, &gh).await;

  debug!("Got users: {}", user_map.len());

  let repo_count = process_repositories(&opts, &gh, &user_map).await;

  info!("Got repos: {}", repo_count);
}
