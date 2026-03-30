mod cli;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use cli::auth::{handle_login, load_api_url};

#[derive(Parser)]
#[command(name = "koda", version, about = "Build, deploy & manage your applications from the CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Deploy {
        source: String,
        #[arg(short, long)]
        port: Option<u16>,
        #[arg(short = 'n', long, default_value = "koda-app")]
        name: String,
        #[arg(short, long)]
        env: Vec<String>,
        #[arg(long)]
        pkgs: Vec<String>,
        #[arg(long)]
        build_cmd: Option<String>,
        #[arg(long)]
        start_cmd: Option<String>,
    },
    Ps,
    Logs {
        id: String,
        #[arg(short = 't', long, default_value_t = 100)]
        tail: u32,
    },
    Stop { id: String },
    Login { email: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Deploy { source, port, name, env, pkgs, build_cmd, start_cmd } =>
            cmd_deploy(source, port, name, env, pkgs, build_cmd, start_cmd).await,
        Commands::Ps => cmd_ps().await,
        Commands::Logs { id, tail } => cmd_logs(id, tail).await,
        Commands::Stop { id } => cmd_stop(id).await,
        Commands::Login { email } => handle_login(email).await,
    }
}

fn load_token() -> Result<String> {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".koda");
    path.push("config.json");

    let content = std::fs::read_to_string(&path)
        .context("Not logged in. Run `koda login <email>` first.")?;

    let config: serde_json::Value = serde_json::from_str(&content)?;
    config["token"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid config file"))
        .map(|s| s.to_string())
}

async fn cmd_deploy(
    source: String,
    port: Option<u16>,
    name: String,
    env: Vec<String>,
    pkgs: Vec<String>,
    build_cmd: Option<String>,
    start_cmd: Option<String>,
) -> Result<()> {
    let api_url = load_api_url();
    let token = load_token()?;

    if !std::path::Path::new(&source).exists() {
        anyhow::bail!("Path '{}' does not exist. Check the folder name and try again.", source);
    }

    // Use folder name as app name if default is used
    let name = if name == "koda-app" {
        std::fs::canonicalize(&source)
            .unwrap_or_else(|_| std::path::PathBuf::from(&source))
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("koda-app")
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
            .collect::<String>()
            .to_lowercase()
    } else {
        name
    };

    println!("Compressing {}...", source);
    let tarball_path = std::env::temp_dir().join("koda-deploy.tar.gz");
    create_tarball(&source, tarball_path.to_str().unwrap())?;
    

    println!("Uploading to Koda...");
    let tarball_bytes = std::fs::read(&tarball_path)?;

    let client = reqwest::Client::new();
    let part = reqwest::multipart::Part::bytes(tarball_bytes)
        .file_name("source.tar.gz")
        .mime_str("application/gzip")?;

    let form = reqwest::multipart::Form::new()
        .part("source", part)
        .text("image_name", name)
        .text("port", port.unwrap_or(3000).to_string())
        .text("env", serde_json::to_string(&env)?)
        .text("pkgs", serde_json::to_string(&pkgs)?)
        .text("build_cmd", build_cmd.unwrap_or_default())
        .text("start_cmd", start_cmd.unwrap_or_default());

    let res = client
        .post(format!("{}/deploy", api_url))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await?;

    let body: serde_json::Value = res.json().await?;

    if let Some(url) = body["url"].as_str() {
        println!("\nDeployed successfully!");
        println!("URL: {}", url);
    } else if let Some(err) = body["error"].as_str() {
        println!("Error: {}", err);
    } else {
        println!("{}", serde_json::to_string_pretty(&body)?);
    }

    Ok(())
}

fn create_tarball(source: &str, output: &str) -> Result<()> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::fs::File;
    use walkdir::WalkDir;

    let skip_dirs = ["node_modules", ".git", "target", ".next", ".nuxt", "dist", "__pycache__", ".venv", "venv"];

    // On Windows, canonicalize returns UNC paths (\\?\C:\...) which break tar
    // So we use the path as-is but convert separators
    // dunce::canonicalize strips UNC prefix on Windows (\\?\C:\...)
    let source_path = dunce::canonicalize(source)
        .unwrap_or_else(|_| std::path::PathBuf::from(source));
    let tar_gz = File::create(output)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for entry in WalkDir::new(&source_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Skip unwanted directories
        if path.components().any(|c| {
            skip_dirs.contains(&c.as_os_str().to_str().unwrap_or(""))
        }) {
            continue;
        }

        let relative = path.strip_prefix(&source_path)?;
        if relative.as_os_str().is_empty() { continue; }

        // Normalize path separators to forward slashes for tar
        let normalized = relative.to_string_lossy().replace("\\", "/");

        if path.is_file() {
            tar.append_path_with_name(path, &normalized)?;
        } else if path.is_dir() {
            tar.append_dir(&normalized, path)?;
        }
    }

    tar.finish()?;
    Ok(())
}

async fn cmd_ps() -> Result<()> {
    let api_url = load_api_url();
    let token = load_token()?;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/containers", api_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let body: serde_json::Value = res.json().await?;
    println!("{}", serde_json::to_string_pretty(&body)?);
    Ok(())
}

async fn cmd_logs(id: String, _tail: u32) -> Result<()> {
    let api_url = load_api_url();
    let token = load_token()?;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/containers/{}/logs", api_url, id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let body: serde_json::Value = res.json().await?;
    if let Some(logs) = body["logs"].as_array() {
        for line in logs {
            print!("{}", line.as_str().unwrap_or(""));
        }
    }
    Ok(())
}

async fn cmd_stop(id: String) -> Result<()> {
    let api_url = load_api_url();
    let token = load_token()?;
    let client = reqwest::Client::new();

    client
        .post(format!("{}/containers/{}/stop", api_url, id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    println!("Container {} stopped", id);
    Ok(())
}
