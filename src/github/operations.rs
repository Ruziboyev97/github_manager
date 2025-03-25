use crate::github::client::GitHubClient;
use crate::github::models::Repository;
use std::error::Error;
use std::io::{self, Write};

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

pub async fn delete_all_repositories(
    github_client: &GitHubClient,
    repos: &[Repository],
) -> Result<(), Box<dyn Error>> {
    println!("\nWARNING: this will permanentaly delete all listed repository.");
    println!("Type 'Delete all' to confirm deletion of all repos or 'Cancel'");

    let mut confirmation = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut confirmation)?;
    let confirmation = confirmation.trim();

    if confirmation == "Delete all" {
        println!("Deleting all repos...");

        for repo in repos {
            print!("Deleting {}...", repo.name);
            io::stdout().flush()?;

            match delete_repo(github_client, &repo.owner.login, &repo.name).await {
                Ok(_) => println!("deleted"),
                Err(e) => println!("Error: {}", e),
            }
            // Adding small delay
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

        println!("Deleted all repos");
    } else {
        println!("Operation cancelled");
    }

    Ok(())
}

pub async fn delete_specific_repositories(
    github_client: &GitHubClient,
    repos: &[Repository],
) -> Result<(), Box<dyn Error>> {
    println!("\nEnter the numbers of repos to delete, separated by commas (e.g 1,2,3)");

    let mut selection = String::new();
    io::stdin().read_line(&mut selection)?;

    let indices: Result<Vec<usize>, _> = selection
        .trim()
        .split(",")
        .map(|s| s.trim().parse::<usize>().map(|n| n - 1))
        .collect();

    match indices {
        Ok(indices) => {
            let selected_repos: Vec<&Repository> =
                indices.iter().filter_map(|&i| repos.get(i)).collect();

            if selected_repos.is_empty() {
                println!("No valid repos selected");

                return Ok(());
            }

            println!("\nYou have selected following");
            for (i, repo) in selected_repos.iter().enumerate() {
                println!("{}. {}", i + 1, repo.name);
            }

            println!("\nWARNING: This will permanently delete these repositories.");
            println!("Type 'DELETE SELECTED' to confirm, or 'CANCEL' to exit:");

            let mut confirmation = String::new();
            io::stdout().flush()?;
            io::stdin().read_line(&mut confirmation)?;
            let confirmation = confirmation.trim();

            if confirmation == "Delete selected" {
                println!("Deleting selected");

                for repo in selected_repos {
                    print!("Deleting {}...", repo.name);
                    io::stdout().flush()?;

                    match delete_repo(github_client, &repo.owner.login, &repo.name).await {
                        Ok(_) => println!("Success"),
                        Err(e) => println!("Error: {}", e),
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }

                println!("Deletion process completed");
            } else {
                println!("operation cancelled");
            }
        }

        Err(_) => {
            println!("Invalid input. Pleas enter numbers separated by commas. (E.g. 1,2,3)");
        }
    }

    Ok(())
}
