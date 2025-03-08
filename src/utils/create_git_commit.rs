//use super::validate_commit_message;
use anyhow::Result;
use std::process::Command;

pub fn create_git_commit(message: &str) -> Result<()> {
    //if !validate_commit_message(message) {
    //    return Err(anyhow::anyhow!("Invalid commit message format"));
    //}

    let output = Command::new("git")
        .args(&["commit", "-m", message])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute git commit: {}", e))?;

    if !output.status.success() {
        let var_name = anyhow::anyhow!("Git commit failed with code: {:?}", output.status.code());
        return Err(var_name);
    }

    Ok(())
}

// TODO: improve error hanling
