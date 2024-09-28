use git2::Repository;
use log::{info, warn};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub trait DataManager {
    fn download(&self) -> Result<(), Box<dyn Error>>;
    fn walk(&self) -> Result<(), Box<dyn Error>>;
}

pub struct GitHubRepoManager {
    repo_id: String,
    commit_hash: Option<String>,
    local_dir: PathBuf,
    local_path: PathBuf,
    log_dir: PathBuf,
    access_token: Option<String>,
}

impl GitHubRepoManager {
    pub fn new(
        repo_id: &str,
        commit_hash: Option<String>,
        local_dir: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        let local_dir = PathBuf::from(local_dir.unwrap_or("/tmp/"));
        if !local_dir.exists() {
            fs::create_dir_all(&local_dir)?;
        }

        let local_path = local_dir.join(repo_id);
        let log_dir = local_dir.join("logs").join(repo_id);
        if !log_dir.exists() {
            fs::create_dir_all(&log_dir)?;
        }

        let access_token = Some(String::from(""));

        Ok(GitHubRepoManager {
            repo_id: repo_id.to_string(),
            commit_hash,
            local_dir,
            local_path,
            log_dir,
            access_token,
        })
    }

    fn is_public(&self) -> Result<bool, Box<dyn Error>> {
        let client = Client::new();
        let url = format!("https://api.github.com/repos/{}", self.repo_id);
        let response = client.get(&url).send()?;

        // Log the response status
        let status = response.status();
        log::info!("Response status for {}: {}", self.repo_id, status);

        match status {
            StatusCode::OK => Ok(true),         // 200 OK means the repo is public
            StatusCode::NOT_FOUND => Ok(false), // 404 Not Found means the repo does not exist
            StatusCode::FORBIDDEN => {
                log::warn!("Access forbidden for repository {}", self.repo_id);
                Ok(false) // 403 Forbidden means you don't have access
            }
            _ => {
                log::error!("Unexpected status for {}: {}", self.repo_id, status);
                Err(Box::from(format!("Unexpected status: {}", status)))
            }
        }
    }

    fn default_branch(&self) -> Result<String, Box<dyn Error>> {
        let client = Client::new();
        let url = format!("https://api.github.com/repos/{}", self.repo_id);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/vnd.github.v3+json".parse()?);

        if let Some(ref token) = self.access_token {
            headers.insert("Authorization", format!("token {}", token).parse()?);
        }

        let response = client.get(&url).headers(headers).send()?;

        if response.status().is_success() {
            let json: HashMap<String, String> = response.json()?;
            Ok(json
                .get("default_branch")
                .unwrap_or(&"main".to_string())
                .to_string())
        } else {
            warn!(
                "Unable to fetch default branch for {}: {}",
                self.repo_id,
                response.status()
            );
            Ok("main".to_string())
        }
    }
}

impl DataManager for GitHubRepoManager {
    fn download(&self) -> Result<(), Box<dyn Error>> {
        if self.local_path.exists() {
            info!("Repository already cloned.");
            return Ok(());
        }

        let is_public = self.is_public()?;
        print!("Repository is public: {}", is_public);
        if !is_public && self.access_token.is_none() {
            return Err(Box::from(
                "Repository is private and no access token is provided",
            ));
        }

        let clone_url = if let Some(ref token) = self.access_token {
            format!("https://{}@github.com/{}.git", token, self.repo_id)
        } else {
            format!("https://github.com/{}.git", self.repo_id)
        };

        // Git cloning logic using the `git2` crate
        info!("Cloning repository from {}", clone_url);

        // Attempt to clone the repository and handle any errors that occur
        match Repository::clone(&clone_url, &self.local_path) {
            Ok(_) => {
                // info!("Repository successfully cloned.");
                Ok(())
            }
            Err(e) => {
                // Return detailed error message if the clone fails
                eprintln!(
                    "Failed to clone repository from {}. Error: {}",
                    clone_url, e
                );
                Err(Box::new(e))
            }
        }
    }

    fn walk(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
