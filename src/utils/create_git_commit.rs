use anyhow::{Context, Result};
use dialoguer::Editor;
use std::process::Command;

pub fn create_git_commit(message: &str) -> Result<()> {
    let editor = get_git_editor()?;

    // Present editable interface
    let edited_message = Editor::new()
        .executable(&editor)
        .extension(".commitmsg")
        .edit(message)
        .unwrap()
        .unwrap();

    println!("I don't now why here though");
    // Clean up the message
    let final_message = process_commit_message(&edited_message);

    // Execute the commit
    let output = Command::new("git")
        .args(["commit", "-m", &final_message])
        .output()
        .context("Failed to execute git commit")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git commit failed:\n{}", stderr);
    }
    Ok(())
}

fn process_commit_message(raw: &str) -> String {
    raw.lines()
        // Remove comment lines and trim whitespace
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn get_git_editor() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["config", "--get", "core.editor"])
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        // Fallback to standard environment variables
        let editor = std::env::var("VISUAL")
            .or_else(|_| std::env::var("EDITOR"))
            .unwrap_or_else(|_| {
                if cfg!(windows) {
                    "notepad".to_string()
                } else {
                    "vi".to_string()
                }
            });
        Ok(editor)
    }
}

// TODO: improve error hanling
