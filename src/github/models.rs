use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub name: String,
    pub owner: Owner,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub login: String,
}
