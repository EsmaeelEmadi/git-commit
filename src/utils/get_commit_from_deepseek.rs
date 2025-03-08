use std::{process, thread::Result};

use crate::types::{ChatRequest, ChatResponse, Config, Message};

pub async fn get_commit_from_deepseek(
    description: &Option<String>,
    config: &Config,
    guideline: &String,
    diff: &String,
) -> Result<String> {
    let client = reqwest::Client::new();

    let desc: String = match description {
        None => String::new(),
        _ => description.clone().expect("Error cloning the description"),
    };

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: guideline.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: format!(
                "note:Generate commit message for these changes\n\n\
                note: if there is a description provided, try to create the commit message around it\n\n
                **Description**\n{}\n\n\
                **Git Diff**\n```diff\n{}\n```",
                desc, diff
            ),
        },
    ];

    let request_body = ChatRequest {
        messages,
        model: "deepseek-chat".to_string(),
        temperature: 0.7,
        max_tokens: Some(1024),
    };

    let response = match client
        .post("https://api.deepseek.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
    {
        Err(e) => {
            println!("error, {}", e);
            process::exit(1);
        }
        Ok(o) => o,
    };

    let response_body: ChatResponse = match response.json().await {
        Err(e) => {
            println!("error, {}", e);
            process::exit(1);
        }
        Ok(o) => o,
    };

    Ok(response_body.choices[0].message.content.clone())
}
