use actix_web::{HttpResponse, web, error};
use reqwest::{Response};
use log::{error};
use crate::{constants::{BOT_ID, QUOTES_D2}, models::{WebhookEvent, MessageRequest}};
use std::error::Error;
use rand::Rng;

// Assuming CharacterQuote and QUOTES_D2 are defined and available here

fn print_random_quote() -> &'static str {
    let mut rng = rand::thread_rng();
    let quote = &QUOTES_D2[rng.gen_range(0..QUOTES_D2.len())];
    let index = rng.gen_range(0..QUOTES_D2.len()); 
    println!("index:{}", index);
    return quote.quote
}

use super::send_message;

async fn handle_quote(target_id: &str, msg_id: &str, nonce: &str) -> Result<Response, Box<dyn std::error::Error>> {
    // Implement your logic to handle the "/quote" trigger
    let message = MessageRequest {
        message_type: Some(1), // Set the message type as desired
        target_id: target_id.to_owned(), // Set the target ID
        content: print_random_quote().to_owned(), // Set the message content
        quote: Some(msg_id.to_owned()), // Set the quoted message ID if applicable
        nonce: Some(nonce.to_owned()), // Set the nonce if applicable
        temp_target_id: None, // Set the temporary target ID if applicable
    };
    let response = send_message(message).await;
    return response;
}

async fn handlePvp(content: &str) {
    // Implement your logic to handle the "/pvp" trigger
    println!("Handling PVP: {}", content);
}

pub async fn message_handler(webhook_event: WebhookEvent) -> Result<HttpResponse, actix_web::Error> {
    let content = &webhook_event.d.content;
    // Check if the content has the expected prefix
    let prefix = "(met)".to_owned() + BOT_ID + "(met)";
    if !content.starts_with(&prefix) {
        return Ok(HttpResponse::NoContent().finish());
    }

    // Remove the prefix from the content
    let content = content.trim_start_matches(&prefix);

    // Check for different triggers and call the corresponding handler functions
    if content.starts_with(" /quote") {
        let response = handle_quote(&webhook_event.d.target_id, &webhook_event.d.msg_id, &webhook_event.d.nonce ).await;
        match response {
            Ok(response) => {
                let status = response.status();
                let headers = response.headers().clone();
                let body = response.bytes().await.map_err(|err| {
                    let std_err: Box<dyn Error> = Box::new(err);
                    actix_web::Error::from(std_err)
                })?;
                let mut http_response = HttpResponse::build(status);
                for (name, value) in headers.iter() {
                    http_response.set_header(name.clone(), value.clone());
                }
                return Ok(http_response.body(body));
            }
            Err(error) => {
                error!("Failed to handle quote: {}", error);
                return Err(error::ErrorInternalServerError(error));
            }
        }
    }

    return Ok(HttpResponse::NoContent().finish())
}