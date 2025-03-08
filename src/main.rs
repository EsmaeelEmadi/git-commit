mod errors;
mod types;
mod utils;

use std::process;

use clap::Parser;
use spinners::{Spinner, Spinners};
use utils::{create_git_commit, get_commit_from_deepseek, run_pre_commits};

use crate::utils::{get_git_diff, is_git_repo, read_config, read_guideline};

#[tokio::main]
async fn main() {
    let mut spinner = Spinner::new(Spinners::Dots9, "Processing...".into());
    let args = types::Args::parse();

    if is_git_repo() {
        let config = match read_config(&args) {
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            Ok(o) => o,
        };

        let guideline = match read_guideline(&args) {
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            Ok(o) => o,
        };

        let diff = match get_git_diff() {
            Ok(diff) if !diff.is_empty() => diff,
            Ok(_) => panic!("\nNo changes in Git repository"),
            Err(e) => panic!("\nError getting Git diff: {}", e),
        };

        match run_pre_commits(&config) {
            Err(e) => panic!("\nError running pre-commits: {}", e),
            Ok(true) => {}
            Ok(false) => {
                process::exit(1);
            }
        }

        let response =
            match get_commit_from_deepseek(&args.description, &config, &guideline, &diff).await {
                Err(_e) => {
                    println!("error");
                    process::exit(1);
                }
                Ok(o) => o,
            };

        spinner.stop_with_newline();

        if let Err(e) = create_git_commit(&response) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        println!("Commit created successfully");
        process::exit(0);
    } else {
        println!("\nNot a Git repository");
        process::exit(1);
    }
}
