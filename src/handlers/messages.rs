use reqwest::{Response};
use log::{info, error};

use crate::{models::{MessageRequest}, constants::{BASE_API_URL, CREATE_MESSAGE}, utils::send_request};

pub async fn send_message(message: MessageRequest) -> Result<Response, Box<dyn std::error::Error>> {
    let url = BASE_API_URL.to_owned() + CREATE_MESSAGE; // Replace with your API URL

    let body = serde_json::to_string(&message)?;
    info!("Message Body: {}", body);
    let response_result = send_request(&url, None, Some(&body)).await;
    info!("Response Result: {:?}", response_result);

    match response_result {
        Ok(response) => {
            info!("Message sent successfully");
            Ok(response)
        }
        Err(error) => {
            error!("Failed to send message: {}", error);
            Err(Box::new(error))
        }
    }
}


// yu_bot::handlers::messages] Response Result: Err(reqwest::Error { kind: Status(400), 
 //   url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("www.kookapp.cn")),
 // port: None, path: "/api/v3/message/create", query: None, fragment: None } })