use chrono_humanize::{Accuracy, HumanTime, Tense};
use failure::Error;
use prettytable::*;
use serde::*;

#[derive(Deserialize, Debug)]
struct Env {
    github_api_token: String,
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config: Env = envy::from_env()?;

    let pull_requests = github::pull_requests::get(config.github_api_token)?;

    let mut table = prettytable::Table::new();

    table.add_row(row!(b => "Oldest open Pull Request", "Open for", "URL"));

    for pr in &pull_requests
        .repository
        .expect("missing repository")
        .pull_requests
        .nodes
        .expect("issue nodes is null")
    {
        if let Some(pull_request) = pr {
            let open_for = chrono::Utc::now().signed_duration_since(pull_request.created_at);

            table.add_row(row!(
                pull_request.title,
                HumanTime::from(open_for).to_text_en(Accuracy::Rough, Tense::Present),
                pull_request.url
            ));
        }
    }

    table.printstd();
    Ok(())
}
