mod github;
mod gui;

use gui::GithubManagerGui;
use iced::{Application, Settings};

#[tokio::main]
async fn main() -> iced::Result {
    GithubManagerGui::run(Settings::default())
}
