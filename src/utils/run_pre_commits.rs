use crate::types::Config;
use anyhow::Result;
use std::process::Command;

pub fn run_pre_commits(config: &Config) -> Result<bool> {
    match &config.pre_commit {
        None => return Ok(true),
        Some(list) => {
            if list.len() == 0 {
                return Ok(true);
            } else {
                for cmd in list {
                    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
                    if !status.success() {
                        return Ok(false);
                    }
                }
                return Ok(true);
            }
        }
    }
}
