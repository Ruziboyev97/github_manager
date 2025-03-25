use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Repository {
    pub name: String,
    pub owner: Owner,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Owner {
    pub login: String,
}
