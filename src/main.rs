use serde::{Deserialize, Serialize};
use std::env;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

const API_BASE_URL: &str = "https://api.github.com";

#[derive(Debug, Deserialize, Clone)]
struct Repository {
    name: String,
    html_url: String,
}

#[derive(Debug, Deserialize)]
struct Contributor {
    contributions: u64,
    repository: Repository,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<Contributor>,
}

fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a GitHub username as the first argument.");
        std::process::exit(1);
    }

    let username = &args[1];
    let url = format!("{}/users/{}/repos?type=owner&per_page=1000", API_BASE_URL, username);

    let mut headers = HeaderMap::new();
    let token = env::var("GITHUB_ACCESS_TOKEN").expect("GITHUB_ACCESS_TOKEN must be set");
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let response = client.get(&url).send()?.json::<Vec<Repository>>()?;

    let mut repos = vec![];
    for repo in response {
        repos.push(repo);
    }

    println!("Latest repositories contributed to by {}:", username);
    for repo in repos {
        println!("- {} ({})", repo.name, repo.html_url);
    }

    Ok(())
}

