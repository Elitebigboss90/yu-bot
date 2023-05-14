use reqwest::{Client, Error, Response};

use crate::constants::TOKEN;

pub async fn send_request(
    api_endpoint: &str,
    parameters: Option<&[(&str, &str)]>,
    body: Option<&str>,
) -> Result<Response, Error> {
    let client = Client::new();

    let mut request_builder = client.post(api_endpoint);

    if let Some(params) = parameters {
        request_builder = request_builder.query(params);
    }

    if let Some(body) = body {
        request_builder = request_builder.body(body.to_owned());
    }

    let response = request_builder
        .header("Content-type", "application/json")
        .header("Authorization", format!("Bot {}", TOKEN.to_owned()))
        .send()
        .await?
        .error_for_status()?;    
    
    Ok(response)
}