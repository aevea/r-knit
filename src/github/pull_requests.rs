use failure::Error;
use graphql_client::*;
use log::*;

type DateTime = chrono::DateTime<chrono::Utc>;
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema/schema.public.graphql",
    query_path = "src/queries/oldest_pr.graphql",
    response_derives = "Debug"
)]
struct OldestPullRequestQuery;

pub fn get(token: String) -> Result<oldest_pull_request_query::ResponseData, Error> {
    let query = OldestPullRequestQuery::build_query(oldest_pull_request_query::Variables {
        owner: "productboard".to_string(),
        name: "pb-frontend".to_string(),
    });

    let client = reqwest::Client::new();

    let mut res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .json(&query)
        .send()?;

    let response_body: Response<oldest_pull_request_query::ResponseData> = res.json()?;
    info!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    }

    let response_data: oldest_pull_request_query::ResponseData =
        response_body.data.expect("missing response data");

    Ok(response_data)
}
