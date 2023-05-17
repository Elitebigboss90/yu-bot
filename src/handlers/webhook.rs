use actix_web::{post, HttpResponse, Responder, web};
use flate2::read::ZlibDecoder;
use std::{io::Read, sync::Arc};
use crate::{models::{WebhookEvent, ChallengeResponse, EncryptedMessage}, constants::ENCRYPT_KEY, utils::decrypt_message, handlers::{send_message, message_handler}, database::GameDatabase};
use serde_json;
use log::{info, error};

pub async fn receive_webhook(body: actix_web::web::Bytes, game_database: web::Data<Arc<GameDatabase>>) -> impl Responder {
    // Decompress the body using zlib
    let mut decoder = ZlibDecoder::new(&body[..]);
    let mut decompressed_body = String::new();
    if let Err(e) = decoder.read_to_string(&mut decompressed_body) {
        error!("Failed to decompress: {}", e);
        return HttpResponse::InternalServerError().body("Failed to decompress");
    }

    info!("Received body: {}", decompressed_body);

    // Try to decrypt the message
    let encrypted_message: EncryptedMessage = match serde_json::from_str(&decompressed_body) {
        Ok(req) => req,
        Err(e) => {
            error!("Invalid JSON: {}", e);
            return HttpResponse::BadRequest().body("Invalid JSON");
        },
    };

    info!("encrypt body: {}", encrypted_message.encrypt);
    let decrypted_message = match decrypt_message(&ENCRYPT_KEY, &encrypted_message.encrypt) {
        Ok(decrypted) => decrypted,
        Err(e) => {
            error!("Failed to decrypt message: {}", e);
            return HttpResponse::InternalServerError().body("Failed to decrypt message");
        },
    };
    info!("Decrypted message: {}", decrypted_message);

    // Deserialize the JSON body
    let webhook_event: WebhookEvent = match serde_json::from_str(&decrypted_message) {
        Ok(event) => event,
        Err(e) => {
            error!("Invalid JSON: {}", e);
            return HttpResponse::BadRequest().body("Invalid JSON");
        },
    };
    
    if webhook_event.d.type_ == 255 {
        return HttpResponse::Ok().finish()
    }

    if webhook_event.d.channel_type == "WEBHOOK_CHALLENGE" {
        match webhook_event.d.challenge {
            Some(challenge_data) => {
                let response = ChallengeResponse {
                    challenge: challenge_data,
                };
                info!("Sending response: {:?}", response);
                return HttpResponse::Ok().json(response)
            },
            None => {
                error!("Invalid request type");
                return HttpResponse::InternalServerError().body("There is no challengeData");
            },
        }
    } else {
        match message_handler(webhook_event, game_database).await {
            Ok(response) => {
                // Successful response
                // Handle the response here
                println!("Response: {:?}", response);
                return HttpResponse::Ok().finish()
            }
            Err(error) => {
                // Error occurred
                // Handle the error here
                println!("Error: {}", error);
                return HttpResponse::InternalServerError().body("Failed to Send Message");
            }
        }
    }
}