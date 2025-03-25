use iced::{
    widget::{button, column, container, row, text, text_input, Column},
    Application, Command, Element, Length, Theme,
};

use crate::github::{client::GitHubClient, models::Repository, operations};

#[derive(Debug, Clone)]
pub enum Message {
    TokenInput(String),
    UsernameInput(String),
    Connect,
    RepositoriesLoaded(Result<Vec<Repository>, String>),
    DeleteSelected,
    ToggleRepository(usize),
    ConfirmDeletion,
    CancelDeletion,
    RepositoryDeleted(Result<(), String>),
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
        String::from("GitHub Repository Manager")
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

                let token = self.token.clone();
                let username = self.username.clone();

                Command::perform(
                    async move {
                        let client =
                            GitHubClient::new(&token, &username).map_err(|e| e.to_string())?;

                        operations::get_all_repos(&client)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    Message::RepositoriesLoaded,
                )
            }
            Message::RepositoriesLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(repos) => {
                        self.repositories = repos;
                        self.selected_repos = vec![false; self.repositories.len()];
                        self.error_message = None;
                        // Create new client after successful load
                        self.client = GitHubClient::new(&self.token, &self.username).ok();
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                    }
                }
                Command::none()
            }
            Message::ToggleRepository(index) => {
                if let Some(selected) = self.selected_repos.get_mut(index) {
                    *selected = !*selected;
                }
                Command::none()
            }
            Message::DeleteSelected => {
                self.show_confirmation = true;
                Command::none()
            }
            Message::ConfirmDeletion => {
                self.show_confirmation = false;

                let selected_repos: Vec<(String, String)> = self
                    .repositories
                    .iter()
                    .zip(self.selected_repos.iter())
                    .filter(|(_, &selected)| selected)
                    .map(|(repo, _)| (repo.owner.login.clone(), repo.name.clone()))
                    .collect();

                if selected_repos.is_empty() {
                    return Command::none();
                }

                if let Some(client) = self.client.clone() {
                    Command::perform(
                        async move {
                            for (owner, name) in selected_repos {
                                if let Err(e) =
                                    operations::delete_repo(&client, &owner, &name).await
                                {
                                    return Err(e.to_string());
                                }
                            }
                            Ok(())
                        },
                        Message::RepositoryDeleted,
                    )
                } else {
                    Command::none()
                }
            }
            Message::CancelDeletion => {
                self.show_confirmation = false;
                Command::none()
            }
            Message::RepositoryDeleted(result) => {
                match result {
                    Ok(_) => {
                        // Refresh the repository list
                        let token = self.token.clone();
                        let username = self.username.clone();

                        Command::perform(
                            async move {
                                let client = GitHubClient::new(&token, &username)
                                    .map_err(|e| e.to_string())?;

                                operations::get_all_repos(&client)
                                    .await
                                    .map_err(|e| e.to_string())
                            },
                            Message::RepositoriesLoaded,
                        )
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                        Command::none()
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if self.client.is_none() && self.repositories.is_empty() {
            // Login view
            let token_input = text_input("Enter GitHub Token", &self.token)
                .on_input(Message::TokenInput)
                .password();

            let username_input = text_input("Enter GitHub Username", &self.username)
                .on_input(Message::UsernameInput);

            let connect_button = button("Connect").on_press(Message::Connect);

            column![token_input, username_input, connect_button].spacing(10)
        } else {
            // Repositories view
            let mut repository_list = Column::new().spacing(10);

            if self.loading {
                repository_list = repository_list.push(text("Loading repositories..."));
            } else {
                for (i, repo) in self.repositories.iter().enumerate() {
                    let selected = self.selected_repos.get(i).copied().unwrap_or(false);
                    let checkbox = button(if selected { "☒" } else { "☐" })
                        .on_press(Message::ToggleRepository(i));

                    repository_list = repository_list.push(
                        row![
                            checkbox,
                            text(&repo.name).size(20),
                            text(&repo.html_url).size(16)
                        ]
                        .spacing(20),
                    );
                }
            }

            let mut controls = Column::new().spacing(10);

            if !self.repositories.is_empty() {
                controls = controls
                    .push(button("Delete Selected Repositories").on_press(Message::DeleteSelected));
            }

            if self.show_confirmation {
                controls = controls.push(
                    column![
                        text("Are you sure you want to delete the selected repositories?"),
                        row![
                            button("Confirm").on_press(Message::ConfirmDeletion),
                            button("Cancel").on_press(Message::CancelDeletion)
                        ]
                        .spacing(10)
                    ]
                    .spacing(10),
                );
            }

            column![repository_list, controls].spacing(20)
        };

        let content = if let Some(error) = &self.error_message {
            column![
                content,
                text(error).style(iced::Color::from_rgb(1.0, 0.0, 0.0))
            ]
            .spacing(20)
        } else {
            content
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
