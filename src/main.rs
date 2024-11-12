use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};
use reqwest::{self, header::USER_AGENT};
use std::{env, io};

#[tokio::main]
async fn main() {
    let mut url = String::from("https://api.github.com/users/");
    let args: Vec<String> = env::args().collect();
    url += &args[1];
    if args.len() < 2 {
        panic!("Error: Enter GitHub username.");
    }
    let mut terminal = ratatui::init();
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(USER_AGENT, "test")
        .send()
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }

    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
