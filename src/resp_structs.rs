use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub login: String,
    pub name: String,
    pub bio: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
}

#[derive(Deserialize, Debug)]
pub struct GraphRespData {
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub pinned_items: PinnedItems,
    pub contributions_collection: ContributionsCollection,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContributionsCollection {
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContributionCalendar {
    pub total_contributions: u32,
    pub weeks: Vec<Week>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Week {
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContributionDay {
    pub contribution_count: u32,
    pub date: String,
}

#[derive(Deserialize, Debug)]
pub struct PinnedItems {
    pub nodes: Vec<Node>,
}

#[derive(Deserialize, Debug)]
pub struct Node {
    pub name: String,
    pub description: String,
    pub stargazers: Stargazers,
    pub forks: Forks,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Forks {
    pub total_count: u32,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stargazers {
    pub total_count: u32,
}

#[derive(Serialize)]
pub struct GraphQLRequest {
    pub query: String,
}
