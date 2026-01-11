use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("git error: {0}")]
    Git(#[from] git2::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid url: {0}")]
    Url(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRepo {
    pub url: String,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCheckout {
    pub local_path: String,
}

pub struct GitBackend;

impl GitBackend {
    pub fn clone_or_update(repo: &GitRepo, dest: &std::path::Path) -> Result<GitCheckout, RepoError> {
        use git2::Repository;

        if dest.exists() {
            let repo_handle = Repository::open(dest)?;
            let mut remote = repo_handle.find_remote("origin")?;
            remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], None, None)?;
        } else {
            let mut builder = git2::build::RepoBuilder::new();
            if let Some(branch) = &repo.branch {
                builder.branch(branch);
            }
            builder.clone(&repo.url, dest)?;
        }

        Ok(GitCheckout {
            local_path: dest.to_string_lossy().to_string(),
        })
    }
}
