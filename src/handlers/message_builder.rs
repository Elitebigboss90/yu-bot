use actix_web::{HttpResponse, web::{self, Data}, error};
use reqwest::{Response};
use log::{error};
use crate::{constants::{BOT_ID, QUOTES_D2}, models::{WebhookEvent, MessageRequest}, game::Game, database::GameDatabase};
use std::{error::Error, sync::Arc};
use rand::Rng;

use super::send_message;

// Assuming CharacterQuote and QUOTES_D2 are defined and available here

fn print_random_quote() -> &'static str {
    let mut rng = rand::thread_rng();
    let quote = &QUOTES_D2[rng.gen_range(0..QUOTES_D2.len())];
    quote.quote
}

async fn handle_quote(target_id: &str, msg_id: &str, nonce: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let message = MessageRequest {
        message_type: Some(1), // Set the message type as desired
        target_id: target_id.to_owned(), // Set the target ID
        content: print_random_quote().to_owned(), // Set the message content
        quote: Some(msg_id.to_owned()), // Set the quoted message ID if applicable
        nonce: Some(nonce.to_owned()), // Set the nonce if applicable
        temp_target_id: None, // Set the temporary target ID if applicable
    };
    send_message(message).await
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
            Ok(http_response.body(body))
        }
        Err(error) => {
            error!("Failed to handle quote: {}", error);
            Err(error::ErrorInternalServerError(error))
        }
    }
}

async fn handle_game_result(game_result: Result<(), anyhow::Error>, webhook_event: &WebhookEvent, message: &str) -> Result<HttpResponse, actix_web::Error> {
    match game_result {
        Ok(_) => {
            println!("Game completed successfully.");
            let message = MessageRequest {
                message_type: Some(1), // Set the message type as desired
                target_id: webhook_event.d.target_id.clone().unwrap(), // Set the target ID
                content: message.to_owned(), // Set the message content
                quote: webhook_event.d.msg_id.clone(), // Set the quoted message ID if applicable
                nonce: webhook_event.d.nonce.clone(), // Set the nonce if applicable
                temp_target_id: None, // Set the temporary target ID if applicable
            };
            let response = send_message(message).await;
            send_message_response_handler(response).await
        }
        Err(e) => {
            println!("An error occurred: {}", e);
            Err(error::ErrorInternalServerError(e))
        }
    }
}

async fn send_text_message(webhook_event: &WebhookEvent, message: &str) -> Result<HttpResponse, actix_web::Error> {
            let message = MessageRequest {
                message_type: Some(1), // Set the message type as desired
                target_id: webhook_event.d.target_id.clone().unwrap(), // Set the target ID
                content: message.to_owned(), // Set the message content
                quote: webhook_event.d.msg_id.clone(), // Set the quoted message ID if applicable
                nonce: webhook_event.d.nonce.clone(), // Set the nonce if applicable
                temp_target_id: None, // Set the temporary target ID if applicable
            };
            let response = send_message(message).await;
            send_message_response_handler(response).await
}

pub async fn message_handler(webhook_event: WebhookEvent, database: web::Data<Arc<GameDatabase>>) -> Result<HttpResponse, actix_web::Error> {
    let content = webhook_event.d.content.as_deref().unwrap_or("");
    let prefix = format!("(met){}(met)", *BOT_ID);

    if !content.starts_with(&prefix) {
        return Ok(HttpResponse::NoContent().finish());
    }

    let content = content.trim_start_matches(&prefix);
    let game = Game::new(database);

    if content.starts_with(" /quote") {
        let response = handle_quote(webhook_event.d.target_id.as_deref().unwrap_or_default(), webhook_event.d.msg_id.as_deref().unwrap_or_default(), webhook_event.d.nonce.as_deref().unwrap_or_default()).await;
        return send_message_response_handler(response).await;
    } else if content.starts_with(" /pvp") {
        let game_result = game.pvp(webhook_event.d.author_id.as_deref().unwrap_or_default()).await;
        return handle_game_result(game_result, &webhook_event, "游戏结束， 玩家胜利").await;
    } else if content.starts_with(" /register") {
        if let Some(message_extra) = webhook_event.d.extra.as_ref() {
            let user_name = message_extra.author.username.clone();
            let register_result = game.register(webhook_event.d.author_id.clone().unwrap(), user_name).await;
            return handle_game_result(register_result, &webhook_event, "注册成功").await;
        }
    } else {
        return send_text_message(&webhook_event, "阿巴阿巴阿巴巴").await;
    }

    Ok(HttpResponse::NoContent().finish())
}
