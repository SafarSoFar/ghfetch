use reqwest::{
    self,
    header::{AUTHORIZATION, USER_AGENT},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, io};
use termion::{
    color::{self, Reset},
    style,
};

mod resp_structs;
use resp_structs::*;

async fn get_user_info(login: &str) {
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
        println!(
            "User: {} {} ({}) {}",
            color::Fg(color::Red),
            userData.login,
            userData.name,
            color::Fg(color::Reset)
        );
        println!("Bio: {}", userData.bio);
        print!("Followers: {} ", userData.followers);
        println!("Following: {}", userData.following);
        println!("Public repositories: {}", userData.public_repos);
    }
}

async fn get_user_work_info(login: &str, token: &str) {
    let url = "https://api.github.com/graphql".to_string();
    let json_body = r#"
        query {
            user(login: ""#
        .to_string()
        + login
        + r#"") {
                contributionsCollection{
                    contributionCalendar{
                        totalContributions,
                        weeks{
                            contributionDays{
                                contributionCount
                                date
                            }
                        }
                        
                    }
                }
                pinnedItems(first: 6, types: REPOSITORY) {
                    nodes {
                        ... on Repository {
                            name
                            description
                            forks{
                                totalCount
                            }
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

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(USER_AGENT, "ghfetch")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&graph)
        .send()
        .await
        .unwrap();

    let graph_data = response.text().await.unwrap();

    let graph_resp_data: GraphRespData = serde_json::from_str(&graph_data).unwrap();

    println!(
        "Total contributions {}",
        graph_resp_data
            .data
            .user
            .contributions_collection
            .contribution_calendar
            .total_contributions
    );
    println!("Pinned Repos:");
    for node in graph_resp_data.data.user.pinned_items.nodes {
        println!("Repo: {} ", node.name);
        println!("{}", node.description);
        println!(
            r#" * {} \|/ {}"#,
            node.stargazers.totalCount, node.forks.totalCount
        );
        println!();
    }
    for week in graph_resp_data
        .data
        .user
        .contributions_collection
        .contribution_calendar
        .weeks
    {
        for day in week.contribution_days {
            print!("{}", day.contribution_count);
        }
        println!();
    }
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

    get_user_info(&login.to_string()).await;
    get_user_work_info(&login.to_string(), &token.to_string()).await;
}
