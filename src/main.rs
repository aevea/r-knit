use failure::Error;
use graphql_client::*;
use log::*;
use prettytable::*;
use serde::*;

type URI = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema/schema.public.graphql",
    query_path = "src/queries/repository.graphql",
    response_derives = "Debug"
)]
struct RepoView;

#[derive(Deserialize, Debug)]
struct Env {
    github_api_token: String,
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config: Env = envy::from_env()?;

    let query = RepoView::build_query(repo_view::Variables {
        owner: "outillage".to_string(),
        name: "commitsar".to_string(),
    });

    let client = reqwest::Client::new();

    let mut res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(config.github_api_token)
        .json(&query)
        .send()?;

    let response_body: Response<repo_view::ResponseData> = res.json()?;
    info!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    }

    let response_data: repo_view::ResponseData = response_body.data.expect("missing response data");

    let stars: Option<i64> = response_data
        .repository
        .as_ref()
        .map(|repo| repo.stargazers.total_count);

    println!(
        "{}/{} - 🌟 {}",
        "outillage",
        "commitsar",
        stars.unwrap_or(0),
    );

    let mut table = prettytable::Table::new();

    table.add_row(row!(b => "issue", "opened_at", "closed_at"));

    for issue in &response_data
        .repository
        .expect("missing repository")
        .pull_requests
        .nodes
        .expect("issue nodes is null")
    {
        if let Some(pull_request) = issue {
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
