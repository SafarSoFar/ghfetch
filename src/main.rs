use console::style;
use reqwest::{
    self,
    header::{AUTHORIZATION, USER_AGENT},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
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

#[derive(Deserialize, Debug)]
struct GraphRespData {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    pinned_items: PinnedItems,
}

#[derive(Deserialize, Debug)]
struct PinnedItems {
    nodes: Vec<Node>,
}

#[derive(Deserialize, Debug)]
struct Node {
    name: String,
    stargazers: Stargazers,
}

#[derive(Deserialize, Debug)]
struct Stargazers {
    totalCount: u32,
}

#[derive(Serialize)]
struct GraphQLRequest {
    query: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => panic!("Error: Enter GitHub username and Token"),
        2 => panic!("Error: Enter  GitHub Token"),
        _ => {}
    }

    // Starting from index 1 because the first argument is the binary location
    let login: String = args[1].clone();
    let token: String = args[2].clone();

    let mut url = String::from("https://api.github.com/users/");
    url += &login;

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
    let json_body = r#"
        query {
            user(login: ""#
        .to_string()
        + &login.to_string()
        + r#"") {
                pinnedItems(first: 6, types: REPOSITORY) {
                    nodes {
                        ... on Repository {
                            name
                            stargazers{
                                totalCount     
                            }
                        }
                    }
                }
            }
        }
    "#;
    let graph = GraphQLRequest {
        query: json_body.to_string(),
    };
    response = client
        .post(url)
        .header(USER_AGENT, "ghfetch")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&graph)
        .send()
        .await
        .unwrap();

    let graph_data = response.text().await.unwrap();

    println!("{}", graph_data);
    let graph_resp_data: GraphRespData = serde_json::from_str(&graph_data).unwrap();
    println!("Pinned Repos:");
    for node in graph_resp_data.data.user.pinned_items.nodes {
        println!(
            "Name: {} {}: {} ",
            style(node.name).cyan(),
            style("*").yellow(),
            style(node.stargazers.totalCount).cyan()
        );
    }
    println!();
}
