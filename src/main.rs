mod github;

use github::client::GitHubClient;
use github::models::Repository;
use github::operations::{delete_repo, get_all_repos};
use std::f128::consts::E;
use std::io::{self, stdout, Write};

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

    let github_client = GitHubClient::new(&token, &username);

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
    io:stdout().flush()?;

    //Get user choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    match choice {
        "1" => {delete_all_repositories(&github_client, &repos).await?;},
        "2" => {delete_specific_repositories(&github_client, &repos).await?;},
        "3" | _ => {println!("Exiting program.");}
        }

        Ok(())
}


