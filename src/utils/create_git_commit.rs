use super::validate_commit_message;
use anyhow::Result;
use std::process::Command;

pub fn create_git_commit(message: &str) -> Result<()> {
    if !validate_commit_message(message) {
        return Err(anyhow::anyhow!("Invalid commit message format"));
    }

    let output = Command::new("git")
        .args(&["commit", "-m", message])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to execute git commit: {}", e))?;

    if !output.success() {
        return Err(anyhow::anyhow!(
            "Git commit failed with exit code: {:?}",
            output.code()
        ));
    }

    Ok(())
}

// TODO: improve error hanling
