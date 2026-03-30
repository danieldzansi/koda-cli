use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use std::io::{self, Write};
use std::fs;

#[derive(Serialize)]
struct LoginReq { email: String }

#[derive(Deserialize)]
struct LoginRes { session_id: String }

pub async fn handle_login(email: String) -> Result<()> {
    let api_url = load_api_url();
    let client = Client::new();

    println!("Requesting magic link for {}...", email);

    let res = client.post(format!("{}/auth/login", api_url))
        .json(&LoginReq { email })
        .send()
        .await?
        .json::<LoginRes>()
        .await?;

    println!("Check your email for the magic link!");
    print!("Waiting for verification");
    io::stdout().flush()?;

    let mut token: Option<String> = None;
    while token.is_none() {
        let poll_res = client.get(format!("{}/auth/poll/{}", api_url, res.session_id))
            .send()
            .await?
            .json::<Option<String>>()
            .await?;

        if let Some(t) = poll_res {
            token = Some(t);
        } else {
            print!(".");
            io::stdout().flush()?;
            sleep(Duration::from_secs(2)).await;
        }
    }

    let final_token = token.unwrap();
    save_config(&final_token, &api_url)?;

    println!("\nLogin successful! Token saved to ~/.koda/config.json");
    Ok(())
}

pub fn load_api_url() -> String {
    // Check config file first
    if let Ok(config) = load_config_file() {
        if let Some(url) = config["api_url"].as_str() {
            return url.to_string();
        }
    }
    // Fall back to env var or default
    std::env::var("API_URL")
        .unwrap_or_else(|_| "https://api.danieldzansi.me".to_string())
}

fn load_config_file() -> anyhow::Result<serde_json::Value> {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".koda");
    path.push("config.json");
    let content = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

fn save_config(token: &str, api_url: &str) -> anyhow::Result<()> {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".koda");
    fs::create_dir_all(&path)?;
    path.push("config.json");

    let config = serde_json::json!({
        "token": token,
        "api_url": api_url
    });
    fs::write(path, serde_json::to_string_pretty(&config)?)?;
    Ok(())
}
