mod github;

use github::client::GitHubClient;
use github::models::Repository;
use github::operations::{delete_repo, get_all_repos};
use std::error::Error;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Github Repository Manager");
    println!("------------------------");

    //Prompt for Github token(github token so'rash)
    print!("Enter your Github Personal Access Toekn");
    io::stdout().flush()?;
    let mut token = String::new();
    io::stdin().read_line(&mut token)?;
    let token = token.trim().to_string();

    //validate token

    if token.is_empty() {
        return Err("Github token cannot be empty".into());
    }

    print!("Enter your Github username");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    // Validate username

    if username.is_empty() {
        return Err("Github username cannot be empty".into());
    }

    let github_client = match GitHubClient::new(&token, &username) {
        Ok(client) => client,
        Err(e) => {
            eprint!("Failed to create github client {}", e);
            return Err(e);
        }
    };

    println!("Fetching repositories for user {}", username);
    let repos = get_all_repos(&github_client).await?;

    if repos.is_empty() {
        println!("No repos found for user {}.", username);
        return Ok(());
    }
    println!("Found {} repositories:", repos.len());
    for (i, repo) in repos.iter().enumerate() {
        println!("{}. {} ({})", i + 1, repo.name, repo.html_url);
    }

    println!("\nWhat would you like to do?");
    println!("1. Delete all repositories");
    println!("2. Delete specific repositories");
    println!("3. Exit");
    print!("Enter your choice (1-3): ");
    io::stdout().flush()?;

    //Get user choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    match choice {
        "1" => {
            delete_all_repositories(&github_client, &repos).await?;
        }
        "2" => {
            delete_specific_repositories(&github_client, &repos).await?;
        }
        "3" | _ => {
            println!("Exiting program.");
        }
    }

    Ok(())
}

//yordamchi funksiyalar
async fn delete_all_repositories(
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

async fn delete_specific_repositories(
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
