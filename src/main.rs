mod github;

use github::client::GitHubClient;
use github::models::Repository;
use github::operations::{get_all_repos, delete_repo};
use std::f128::consts::E;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Github Repository Manager");
    println!("------------------------");

    //Prompt for Github token(github token so'rash)
    print!("Enter your Github Personal Access Toekn")
    io::stdout().flush()?;
    let mut token = String::new();
    io::stdin().read_line(&mut token)?;
    let token = token.trim().to_string();


    //validate token

    if token.is_empty() {
    return Err("Github token cannot be empty".into());
    }

    print!("Enter your Github username")
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

        println!("Found {} repositories:", repos.len());
        for (i, repo) in repos.iter().enumerate() {
            println!("{}. {} ({})", i +1, repo.name, repo.html_url);
        }

        
    }


}