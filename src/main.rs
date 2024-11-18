use reqwest::{
    self,
    header::{AUTHORIZATION, USER_AGENT},
};
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Read, Stdout, Write},
    path::Path,
};
use termion::{
    color::{self},
    cursor::DetectCursorPos,
    raw::{IntoRawMode, RawTerminal},
};

mod resp_structs;
use resp_structs::*;

static mut TERM_SIZE_X: u16 = 32;

fn get_x_space_left(raw_term: &mut RawTerminal<Stdout>) -> usize {
    unsafe {
        let (cur_pos_x, _) = raw_term.cursor_pos().unwrap();
        let space_left = (TERM_SIZE_X as i32 - cur_pos_x as i32) as usize;
        space_left
    }
}

fn trim_to_fit_term(string_to_trim: String, raw_term: &mut RawTerminal<Stdout>) -> String {
    //let (cur_pos_x, _) = match raw_term.cursor_pos() {
    //    Ok(val) => val,
    //    Err(e) => (0, 0),
    //};
    let space_left = get_x_space_left(raw_term);
    string_to_trim.chars().take(space_left).collect()
}

fn custom_print_line(string_to_print: String, raw_term: &mut RawTerminal<Stdout>) {
    print_logo(raw_term);
    let truncated: String = trim_to_fit_term(string_to_print, raw_term);
    println!("{}\r", truncated);
}

fn get_terminal_size_x() {
    unsafe {
        (TERM_SIZE_X, _) = termion::terminal_size().unwrap();
    }
}

const SPACE_OFFSET: &str = "      ";
fn print_logo(raw_term: &mut RawTerminal<Stdout>) {
    unsafe {
        let trimmed_logo_part = trim_to_fit_term(GH_LOGO_VEC[LOGO_INDEX].to_string(), raw_term);
        print!(
            "{}{}{}{}",
            color::Fg(color::White),
            trimmed_logo_part,
            color::Fg(color::Reset),
            SPACE_OFFSET
        );

        // the last index is just empty space offsets
        if LOGO_INDEX < GH_LOGO_VEC.len() - 1 {
            LOGO_INDEX += 1;
        }
    }
}

async fn get_user_info(login: &str, raw_term: &mut RawTerminal<Stdout>) {
    let mut url = String::from("https://api.github.com/users/");
    url += &login;

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(USER_AGENT, "ghfetch")
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let str_data = &response.text().await.unwrap();
        let user_data: UserData = serde_json::from_str(str_data).unwrap();

        println!();

        custom_print_line(
            format!(
                "User: {} {} ({}) {} ",
                color::Fg(color::Red),
                user_data.login,
                user_data.name,
                color::Fg(color::Reset)
            ),
            raw_term,
        );
        //println!(r#"Bio: '{}'"#, userData.bio);
    }
}

static mut LOGO_INDEX: usize = 0;

const GH_LOGO_VEC: [&str; 34] = [
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
    "                                                             ",
    // the last index is the space offset (Hacky stuff, yeah)
];

fn print_border(raw_term: &mut RawTerminal<Stdout>) {
    print_logo(raw_term);

    let border = "_______________________";
    let trimmed_border = trim_to_fit_term(border.to_string(), raw_term);
    println!("{}\r", trimmed_border);
    //print_logo();
    //println!();
}

async fn get_user_work_info(login: &str, token: &str, raw_term: &mut RawTerminal<Stdout>) {
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

    custom_print_line(
        format!(
            "Total contributions {}",
            graph_resp_data
                .data
                .user
                .contributions_collection
                .contribution_calendar
                .total_contributions
        ),
        raw_term,
    );

    for node in graph_resp_data.data.user.pinned_items.nodes {
        custom_print_line(
            format!(
                "{}{}:{}",
                color::Fg(color::Red),
                node.name,
                color::Fg(color::Reset)
            ),
            raw_term,
        );
        custom_print_line(format!("{}", node.description), raw_term);
        custom_print_line(
            format!(
                r#" {}*{} {}   \|/ {}"#,
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                node.stargazers.total_count,
                node.forks.total_count
            ),
            raw_term,
        );

        print_border(raw_term);
    }

    for i in 0..7 {
        print_logo(raw_term);
        for week in graph_resp_data
            .data
            .user
            .contributions_collection
            .contribution_calendar
            .weeks
            .iter()
        {
            if i < week.contribution_days.len() {
                if get_x_space_left(raw_term) <= 0 {
                    break;
                }
                print_activity_square(week.contribution_days[i].contribution_count);
            }
        }
        println!("\r");
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
    let mut raw_term = std::io::stdout().into_raw_mode().unwrap();
    get_terminal_size_x();

    let args: Vec<String> = env::args().collect();

    if is_config_file_exists() {
        let (login, token) = read_config_file().unwrap();

        get_user_info(&login, &mut raw_term).await;
        get_user_work_info(&login, &token, &mut raw_term).await;
    } else {
        match args.len() {
            1 => panic!("Error: Enter GitHub username + token"),
            2 => {
                get_user_info(&args[1], &mut raw_term).await;
                println!("Enter GitHub token for full information")
            }
            3 => {
                println!("Save arguments to the config file?");
                println!("Yes = [Y] No = [N]");
                let mut read_buf = [0u8; 1];
                io::stdin()
                    .read_exact(&mut read_buf)
                    .expect("Couldn't read the input character");

                let ch = read_buf[0] as char;
                let ch = ch.to_lowercase().next().unwrap();
                if ch == 'y' {
                    create_config_file(&args[1], &args[2])
                }

                get_user_info(&args[1], &mut raw_term).await;
                get_user_work_info(&args[1], &args[2], &mut raw_term).await;
            }
            _ => {}
        }
    }

    // Starting from index 1 because the first argument is the binary location

    unsafe {
        while LOGO_INDEX < GH_LOGO_VEC.len() - 1 {
            print_logo(&mut raw_term);
            println!();
        }
    }
}

fn is_config_file_exists() -> bool {
    let file_path = Path::new("config");
    file_path.exists()
}
fn read_config_file() -> io::Result<(String, String)> {
    let mut login = String::new();
    let mut token = String::new();
    let file = File::open("config").expect("Couldn't open the config file");
    let reader = BufReader::new(file);
    for (i, line_content) in reader.lines().enumerate() {
        match i {
            0 => login = line_content?,
            1 => token = line_content?,
            _ => {}
        }
    }
    Ok((login, token))
}

fn create_config_file(login: &str, token: &str) {
    let mut file = File::create("config").expect("Couldn't create config file");
    let _ = file.write(login.as_bytes());
    let _ = file.write(b"\n");
    let _ = file.write(token.as_bytes());
}
