use actix_web::{HttpResponse, web::{self, Data}, error};
use reqwest::{Response};
use log::{error};
use crate::{constants::{BOT_ID, QUOTES_D2}, models::{WebhookEvent, MessageRequest}, game::Game, database::GameDatabase};
use std::{error::Error, sync::Arc};
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

async fn send_message_response_handler(response: Result<Response, Box<dyn std::error::Error>>) ->  Result<HttpResponse, actix_web::Error> {
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

pub async fn message_handler(webhook_event: WebhookEvent, database: web::Data<Arc<GameDatabase>>) -> Result<HttpResponse, actix_web::Error> {
    let content = &webhook_event.d.content.unwrap();
    // Check if the content has the expected prefix
    let prefix = "(met)".to_owned() + &BOT_ID + "(met)";
    if !content.starts_with(&prefix) {
        return Ok(HttpResponse::NoContent().finish());
    }

    // Remove the prefix from the content
    let content = content.trim_start_matches(&prefix);

    // Check for different triggers and call the corresponding handler functions
    if content.starts_with(" /quote") {
        let target_id = webhook_event.d.target_id;
        let target_id: &str = match &target_id {
            Some(s) => s,
            None => "",
        };
        let msg_id = webhook_event.d.msg_id;
        let msg_id: &str = match &msg_id {
            Some(s) => s,
            None => "",
        };
        let nonce = webhook_event.d.nonce;
        let nonce: &str = match &nonce {
            Some(s) => s,
            None => "",
        }; 
        let response = handle_quote(target_id, msg_id, nonce ).await;
        return send_message_response_handler(response).await;
    } else if content.starts_with(" /pvp") {
        // let database = database.clone();
        let game = Game::new(database);
        let user_id = webhook_event.d.author_id;
        let user_id: &str = match &user_id {
            Some(s) => s,
            None => "",
        };
        let game_result = game.pvp(user_id);
        match game_result.await {
            Ok(_) => {
                let target_id = webhook_event.d.target_id;
                let msg_id = webhook_event.d.msg_id;
                let nonce = webhook_event.d.nonce;
                println!("Game completed successfully.");
                let message = MessageRequest {
                    message_type: Some(1), // Set the message type as desired
                    target_id: target_id.unwrap(), // Set the target ID
                    content: "游戏结束， 玩家胜利".to_owned(), // Set the message content
                    quote: msg_id, // Set the quoted message ID if applicable
                    nonce: nonce, // Set the nonce if applicable
                    temp_target_id: None, // Set the temporary target ID if applicable
                };
                let response = send_message(message).await;
                return send_message_response_handler(response).await;
            }
            Err(e) => {
                println!("An error occurred: {}", e);
            }
        }
        
    } else if content.starts_with(" /register") {
        let database = database.clone();
        let game = Game::new(database);
        let author_id = webhook_event.d.author_id.unwrap();
        match webhook_event.d.extra {
            Some(message_extra) => {
                let user_name = message_extra.author.username.to_owned();
                let register_result = game.register(author_id, user_name);
                match register_result.await {
                    Ok(_) => {
                        let target_id = webhook_event.d.target_id;
                        let msg_id = webhook_event.d.msg_id;
                        let nonce = webhook_event.d.nonce;
                        println!("Game completed successfully.");
                        let message = MessageRequest {
                            message_type: Some(1), // Set the message type as desired
                            target_id: target_id.unwrap(), // Set the target ID
                            content: "注册成功".to_owned(), // Set the message content
                            quote: msg_id, // Set the quoted message ID if applicable
                            nonce: nonce, // Set the nonce if applicable
                            temp_target_id: None, // Set the temporary target ID if applicable
                        };
                        let response = send_message(message).await;
                        return send_message_response_handler(response).await;
                    }
                    Err(e) => {
                        println!("An error occurred: {}", e);
                    }
                }
            }
            None => {
                // Handle the case where there is no value
            }
        }
    }

    return Ok(HttpResponse::NoContent().finish())
}

