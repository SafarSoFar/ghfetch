use console::Color;
use reqwest::{
    self,
    header::{AUTHORIZATION, USER_AGENT},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, io};
use termion::{
    color::{self, Reset},
    cursor::{self, DetectCursorPos},
    raw::IntoRawMode,
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

        print_logo();
        println!(
            "User: {} {} ({}) {} ",
            color::Fg(color::Red),
            userData.login,
            userData.name,
            color::Fg(color::Reset)
        );
        //println!(r#"Bio: '{}'"#, userData.bio);
    }
}

static mut logo_index: usize = 0;

const gh_logo_vec: [&str; 33] = [
    "                          @@@@@@@@@                          ",
    "                   @@@@@@@@@@@@@@@@@@@@@@@                   ",
    "               @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@               ",
    "             @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@             ",
    "          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          ",
    "        @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@        ",
    "       @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@       ",
    "     @@@@@@@@@   @@@@@@@@@@@@@@@@@@@@@@@@@@    @@@@@@@@@     ",
    "    @@@@@@@@@@       @@@@@@@     @@@@@@@       @@@@@@@@@@    ",
    "   @@@@@@@@@@@                                 @@@@@@@@@@@   ",
    "  @@@@@@@@@@@@                                 @@@@@@@@@@@@  ",
    "  @@@@@@@@@@@@                                 @@@@@@@@@@@@  ",
    " @@@@@@@@@@@@                                   @@@@@@@@@@@@ ",
    " @@@@@@@@@@@                                     @@@@@@@@@@@ ",
    "@@@@@@@@@@@                                       @@@@@@@@@@@",
    "@@@@@@@@@@@                                       @@@@@@@@@@@",
    "@@@@@@@@@@@                                       @@@@@@@@@@@",
    "@@@@@@@@@@@                                       @@@@@@@@@@@",
    "@@@@@@@@@@@@                                     @@@@@@@@@@@@",
    "@@@@@@@@@@@@                                     @@@@@@@@@@@@",
    " @@@@@@@@@@@@                                   @@@@@@@@@@@@ ",
    " @@@@@@@@@@@@@                                 @@@@@@@@@@@@@ ",
    "  @@@@@@@@@@@@@@                             @@@@@@@@@@@@@@  ",
    "  @@@@@@   @@@@@@@@                       @@@@@@@@@@@@@@@@@  ",
    "   @@@@@@@   @@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@   ",
    "    @@@@@@@@   @@@@@@@@@             @@@@@@@@@@@@@@@@@@@@    ",
    "      @@@@@@@    @@@@@@               @@@@@@@@@@@@@@@@@      ",
    "       @@@@@@                         @@@@@@@@@@@@@@@@       ",
    "         @@@@@@@                      @@@@@@@@@@@@@@         ",
    "           @@@@@@@@@@@@               @@@@@@@@@@@@           ",
    "             @@@@@@@@@@               @@@@@@@@@@             ",
    "                @@@@@@@               @@@@@@@                ",
    "                    @@                 @@                    ",
];

fn print_logo() {
    unsafe {
        print!("{}      ", gh_logo_vec[logo_index]);
        logo_index += 1;
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

    print_logo();
    println!(
        "Total contributions {}",
        graph_resp_data
            .data
            .user
            .contributions_collection
            .contribution_calendar
            .total_contributions
    );
    print_logo();
    println!("Pinned Repos:");
    for node in graph_resp_data.data.user.pinned_items.nodes {
        print_logo();
        println!("Repo: {} ", node.name);
        print_logo();
        println!("{}", node.description);
        print_logo();
        println!(
            r#" * {} \|/ {}"#,
            node.stargazers.total_count, node.forks.total_count
        );
    }

    for i in 0..7 {
        print_logo();
        for week in graph_resp_data
            .data
            .user
            .contributions_collection
            .contribution_calendar
            .weeks
            .iter()
        {
            if i < week.contribution_days.len() {
                //print!("{}", week.contribution_days[i].contribution_count);
                print_activity_square(week.contribution_days[i].contribution_count);
            }
        }
        println!();
    }
}

fn print_activity_square(contribution_count: u32) {
    if contribution_count == 0 {
        print!(
            "{}#{}",
            color::Fg(color::Rgb(47, 52, 59)),
            color::Fg(color::Reset)
        );
        return;
    }
    if contribution_count < 4 {
        print!(
            "{}#{}",
            color::Fg(color::Rgb(14, 68, 41)),
            color::Fg(color::Reset)
        );
        return;
    }
    if contribution_count < 8 {
        print!(
            "{}#{}",
            color::Fg(color::Rgb(0, 109, 50)),
            color::Fg(color::Reset)
        );
        return;
    }
    if contribution_count < 10 {
        print!(
            "{}#{}",
            color::Fg(color::Rgb(38, 166, 65)),
            color::Fg(color::Reset)
        );
        return;
    } else {
        print!(
            "{}#{}",
            color::Fg(color::Rgb(57, 211, 83)),
            color::Fg(color::Reset)
        );
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

    unsafe {
        while (logo_index != gh_logo_vec.len()) {
            print_logo();
            println!();
        }
    }
}
