use failure::Error;
use graphql_client::*;
use log::*;

type DateTime = String;
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema/schema.public.graphql",
    query_path = "src/queries/pull_request.graphql",
    response_derives = "Debug"
)]
struct PullRequestQuery;

pub fn get(token: String) -> Result<pull_request_query::ResponseData, Error> {
    let query = PullRequestQuery::build_query(pull_request_query::Variables {
        owner: "productboard".to_string(),
        name: "pb-frontend".to_string(),
    });

    let client = reqwest::Client::new();

    let mut res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .json(&query)
        .send()?;

    let response_body: Response<pull_request_query::ResponseData> = res.json()?;
    info!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    }

    let response_data: pull_request_query::ResponseData =
        response_body.data.expect("missing response data");

    Ok(response_data)
}
