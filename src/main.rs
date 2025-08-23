use std::env;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize};

const GITLAB_API_BASE: &str = "https://gitlab.com/api/v4";

#[derive(Deserialize)]
struct Group {
    id: u64,
    name: String,
    full_path: String,
}

#[derive(Deserialize)]
struct Project {
    id: u64,
    name: String,
}

async fn fetch_items<T>(client: &reqwest::Client, endpoint: &str) -> Result<Vec<T>,Box<dyn std::error::Error>>
where
    T:for<'de>Deserialize<'de>,
    {
        let url = format!("{}{}", GITLAB_API_BASE, endpoint);
        let response = client.get(&url).send().await?.error_for_status()?;
        let items:Vec<T> = response.json().await?;
        Ok(items)
    }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let pat = env::var("GITLAB_PAT").expect("GITLAB_PAT must be set in your .env file.");
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", pat).parse().unwrap());
    let client = reqwest::Client::builder().default_headers(headers).build()?;
    println!("Fetching all groups ...");
    let groups:Vec<Group> = fetch_items(&client, "/groups?owned=true").await?;
    if groups.is_empty() {
        println!("No groups found for this account.");
    } else {
        println!("\nFound {} groups:", groups.len()); 
        for group in groups {
            println!("\nGroup: {} (ID: {}) (PATH: {})", group.name, group.id, group.full_path);
            let projects_endpoint = format!("/groups/{}/projects", group.id);
            let projects: Vec<Project> = fetch_items(&client, &projects_endpoint).await?;
            if projects.is_empty() {
                println!("No projects found.");
            } else {
                println!("Projects ({})", projects.len());
                for project in projects {
                    println!("Project: {} (ID: {})", project.name, project.id);
                }
            }
        }
    }
    Ok(())
}
