use chrono_humanize::{Accuracy, HumanTime, Tense};
use failure::Error;
use prettytable::*;
use serde::*;
use statistical::{mean, median};

#[derive(Deserialize, Debug)]
struct Env {
    github_api_token: String,
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config: Env = envy::from_env()?;

    let pull_requests =
        github::pull_requests::oldest_pr("fallion", "fallion", config.github_api_token.clone())?;

    let mut table = prettytable::Table::new();

    table.add_row(row!(b => "Oldest open Pull Request", "Open for", "URL"));

    for pr in &pull_requests
        .repository
        .expect("missing repository")
        .pull_requests
        .nodes
        .expect("pull request nodes is null")
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

    // Merged PRs

    let pull_requests =
        github::pull_requests::merged_prs("fallion", "fallion", config.github_api_token.clone())?;

    let mut table = prettytable::Table::new();

    let nodes = &pull_requests
        .repository
        .expect("missing repository")
        .pull_requests
        .nodes
        .expect("pull request nodes is null");

    let mut durations = vec![];

    for pr in nodes {
        if let Some(pull_request) = pr {
            let merged_after = pull_request
                .merged_at
                .expect("merged_at is empty")
                .signed_duration_since(pull_request.created_at);

            durations.push(merged_after.num_seconds() as f64);
        }
    }

    let mean_time_to_merge = mean(&durations);

    let duration = chrono::Duration::seconds(mean_time_to_merge as i64);

    table.add_row(row!(
        "Mean time to merge",
        HumanTime::from(duration).to_text_en(Accuracy::Precise, Tense::Present),
    ));

    let median_time_to_merge = median(&durations);

    let duration = chrono::Duration::seconds(median_time_to_merge as i64);
    table.add_row(row!(
        "Median time to merge",
        HumanTime::from(duration).to_text_en(Accuracy::Rough, Tense::Present),
    ));

    table.printstd();

    Ok(())
}
