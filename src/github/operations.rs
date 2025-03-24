use crate::github::client::GitHubClient;
use crate::github::models::Repository;
use std::error::Error;

// Function to get all repositories for a user
pub async fn get_all_repos(
    github_client: &GitHubClient,
) -> Result<Vec<Repository>, Box<dyn Error>> {
    // Format the URL with the username
    let url = format!(
        "https://api.github.com/users/{}/repos?per_page=100",
        github_client.get_username()
    );

    // Send the GET request
    let response = github_client.get_client().get(&url).send().await?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(format!("Failed to fetch repositories: {}", response.status()).into());
    }

    // Parse the JSON response into our Repository structs
    let repos = response.json::<Vec<Repository>>().await?;
    Ok(repos)
}

// Function to delete a repository
pub async fn delete_repo(
    github_client: &GitHubClient,
    owner: &str,
    repo_name: &str,
) -> Result<(), Box<dyn Error>> {
    // Format the URL for the repository
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo_name);

    // Send the DELETE request
    let response = github_client.get_client().delete(&url).send().await?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(format!(
            "Failed to delete repository {}: {}",
            repo_name,
            response.status()
        )
        .into());
    }

    // Return Ok if the deletion was successful
    Ok(())
}
