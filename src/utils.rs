use inquire::{error::InquireError, ui::{RenderConfig, Styled, Color}};
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum CzenError {
    #[error("Encountered error during prompts: {0}")]
    InquireError(#[from] InquireError),
    #[error("Could not commit: {0}")]
    IoError(#[from] io::Error),
}

pub fn description_render_config() -> RenderConfig {
    RenderConfig::default()
        .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
}

pub fn compose_commit_message(
    commit_type: &str,
    scope: &str,
    breaking_change: bool,
    short_commit_message: &str,
    long_commit_message: &str,
) -> String {
    let mut commit_message = String::new();

    if !scope.is_empty(){
        commit_message.push_str(&format!("{}({})", commit_type, scope));
    }

    if breaking_change {
        commit_message.push_str("!: ");
    } else {
        commit_message.push_str(": ");
    }
    
    commit_message.push_str(short_commit_message);

    if !long_commit_message.is_empty() {
        commit_message.push_str(&format!("\n\n{}", long_commit_message));
    }

    commit_message
}
