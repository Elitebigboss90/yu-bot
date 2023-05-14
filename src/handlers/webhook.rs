use actix_web::{post, HttpResponse, Responder};
use flate2::read::ZlibDecoder;
use std::io::Read;
use crate::models::{ChallengeRequest, ChallengeResponse, EncryptedMessage};
use serde_json;
use log::{info, error};

use crate::utils::decrypt_message;

#[post("/acceptMessage")]
pub async fn receive_webhook(body: actix_web::web::Bytes) -> impl Responder {
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
    let decrypted_message = match decrypt_message("f6KMba1woPml77", &encrypted_message.encrypt) {
        Ok(decrypted) => decrypted,
        Err(e) => {
            error!("Failed to decrypt message: {}", e);
            return HttpResponse::InternalServerError().body("Failed to decrypt message");
        },
    };
    info!("Decrypted message: {}", decrypted_message);

    // Deserialize the JSON body
    let challenge_request: ChallengeRequest = match serde_json::from_str(&decrypted_message) {
        Ok(req) => req,
        Err(e) => {
            error!("Invalid JSON: {}", e);
            return HttpResponse::BadRequest().body("Invalid JSON");
        },
    };

    // Check if it's a challenge request and return the challenge
    if challenge_request.d.channel_type == "WEBHOOK_CHALLENGE" {
        let response = ChallengeResponse {
            challenge: challenge_request.d.challenge,
        };
        info!("Sending response: {:?}", response);
        HttpResponse::Ok().json(response)
    } else {
        error!("Invalid request type");
        HttpResponse::BadRequest().body("Invalid request type")
    }
}