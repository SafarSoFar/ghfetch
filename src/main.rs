use console::style;
use reqwest::{self, header::USER_AGENT};
use serde::{Deserialize, Serialize};
use std::{env, io};

#[derive(Serialize, Deserialize)]
struct UserData {
    login: String,
    name: String,
    bio: String,
    public_repos: u32,
    followers: u32,
    following: u32,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Error: Enter GitHub username.");
    }

    let mut url = String::from("https://api.github.com/users/");
    url += &args[1];

    let client = reqwest::Client::new();
    let mut response = client
        .get(url)
        .header(USER_AGENT, "ghfetch")
        .send()
        .await
        .unwrap();
    if response.status().is_success() {
        let strData = &response.text().await.unwrap();
        let userData: UserData = serde_json::from_str(strData).unwrap();

        println!();
        println!("Login: {}", style(userData.login).cyan());
        println!("Name: {}", style(userData.name).cyan());
        println!("Bio: {}", style(userData.bio).cyan());
        print!("Followers: {} ", style(userData.followers).cyan());
        println!("Following: {}", style(userData.following).cyan());
        println!(
            "Public repositories: {}",
            style(userData.public_repos).cyan()
        );
    }
    url = "https://api.github.com/graphql".to_string();
    let json_body = "
        {
        user(login: '') {
            pinnedItems(first: 6, types: REPOSITORY) {
            nodes {
                ... on Repository {
                name
                }
            }
            }
        }
        }"
    .to_string();

    response = client
        .post(url)
        .header(USER_AGENT, "ghfetch")
        .json(&json_body)
        .send()
        .await
        .unwrap();
    println!("{}", response.text().await.unwrap());
}
