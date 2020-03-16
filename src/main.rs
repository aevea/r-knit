use failure::Error;
use prettytable::*;
use serde::*;
mod github;

#[derive(Deserialize, Debug)]
struct Env {
    github_api_token: String,
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config: Env = envy::from_env()?;

    github::say_hi();
    let pull_requests = github::pull_requests::get(config.github_api_token)?;

    let mut table = prettytable::Table::new();

    table.add_row(row!(b => "issue", "opened_at", "closed_at"));

    for pr in &pull_requests
        .repository
        .expect("missing repository")
        .pull_requests
        .nodes
        .expect("issue nodes is null")
    {
        if let Some(pull_request) = pr {
            let closed_at = match &pull_request.closed_at {
                None => String::from(""),
                Some(x) => x.to_string(),
            };
            table.add_row(row!(
                pull_request.title,
                pull_request.created_at.to_string(),
                closed_at
            ));
        }
    }

    table.printstd();
    Ok(())
}
