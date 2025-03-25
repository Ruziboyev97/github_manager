use iced::{
    widget::{button, column, container, row, text, text_input, Button, Column, Container, Text}, Application, Command, Element, Length, Settings, Theme
};

use crate::github::{client::{self, GitHubClient}, models::Repository, operations};

#[derive(Debug, Clone)]
pub enum Message {
    TokenInput(String),
    UsernameInput(String),
    Connect,
    RepositoriesLoaded(Result<Vec<Repository>, String>),
    DeleteSelected,
    ToggleRepositor(usize),
    ConfirmDeletion,
    CancelDeletion,
    RepositoryDeleted(Result<(), String>)
}

pub struct GithubManagerGui {
    token: String,
    username: String,
    client: Option<GitHubClient>,
    repositories: Vec<Repository>,
    selected_repos: Vec<bool>,
    error_message: Option<String>,
    loading: bool,
    show_confirmation: bool,
}

impl Application for GithubManagerGui {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            GithubManagerGui {
                token: String::new(),
                username: String::new(),
                client: None,
                repositories: Vec::new(),
                selected_repos: Vec::new(),
                error_message: None,
                loading: false,
                show_confirmation: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Github Repository Manager")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TokenInput(value) => {
                self.token = value;
                Command::none()
            }

            Message::UsernameInput(value) => {
                self.username = value;
                Command::none()
            }

            Message::Connect => {
                if self.token.is_empty() || self.username.is_empty() {
                    self.error_message = Some("Token and username are required".to_string());
                    return Command::none();
                }

                match GitHubClient::new(&self.token, &self.username) {
                    Ok(client) => {
                        self.client = Some(client);
                        self.loading = true;
                        self.error_message = None;

                        let client = self.client.as_ref().unwrap().clone();
                        Command::perform(
                            async move {
                                operations::get_all_repos(&client)
                                    .await
                                    .map_err(|e| e.to_string()) 
                            }, Message::RepositoriesLoaded)
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                        Command::none()
                    }
                }
            }

            Message::RepositoriesLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(repos) => {
                        self.repositories = repos;
                        self.selected_repos = vec![false; self.repositories.len()];
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                    }
                }
                Command::none()
            }

            Message::ToggleRepositor(index) => {
                //todo
            }
        }
    }
}
