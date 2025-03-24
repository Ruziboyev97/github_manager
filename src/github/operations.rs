use crate::github::client::GitHubClient;
use crate::github::models::Repository;
use std::error::Error;

pub async fn get_all_repos(github_client: &GitHubClient) -> Result<Vec<Repository>, Box<dyn Error>> {
    let url = format!("https://api.github.com/users/{}/repos?per_page=100",
    github_client.get_username());

    // Send the GET request
    let response = github_client.get_client().get(&url).send().await?;


    // CHek if the request was successful
    if !response.status().is_success() {
        return  Err(format!("Failed to fetch repositories: {}", response.status()).into());
    }

    let repos = response.json::<Vec<Repository>>().await?;
    Ok(repos)
}

//Function to delete a repository(o'chiruvchi funksiya)
pub async fn delete_repo(
    github_client: &GitHubClient,
    owner: &str,
    repo_name: &str
) -> Result<(), Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo_name);

    let response = github_client.get_client().delete(&url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to delete repository {}: {}", repo_name, response.status()).into());
    }

    Ok(())
}