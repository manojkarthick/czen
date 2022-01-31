use inquire::{Select, Confirm, Text, Editor};
use std::process::Command;
use crate::utils::{CzenError, description_render_config, compose_commit_message};

mod utils;

fn main() -> Result<(), CzenError>{

    let commit_types = vec!["feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "revert"];

    let commit_type = Select::new("Select conventional commit type:", commit_types).prompt()?;
    let scope = Text::new("Provide a scope for this change:").prompt()?;
    let breaking_change = Confirm::new("Is this a breaking change?").with_default(false).prompt()?;
    let short_commit_message = Text::new("Provide short commit message:").prompt()?;
    let long_commit_message = Editor::new("Provide long commit message:").with_formatter(&|submission| {
        let char_count = submission.chars().count();
        if char_count == 0 {
            String::from("<skipped>")
        } else if char_count <= 20 {
            submission.into()
        } else {
            let mut substr: String = submission.chars().take(17).collect();
            substr.push_str("...");
            substr
        }
    })
    .with_render_config(description_render_config())
    .prompt()?;

    let commit_message = compose_commit_message(
        &commit_type,
        &scope,
        breaking_change,
        &short_commit_message,
        &long_commit_message,
    );
    
    Command::new("git")
        .args(&["commit", "-m", &commit_message])
        .status()?;

    Ok(())

}
