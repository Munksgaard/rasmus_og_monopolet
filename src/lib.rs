#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use actix_web::HttpResponse;
use actix_web::{http, server, App, Form, Responder};
use log::{error, info};
use reqwest::header::AUTHORIZATION;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    host: Option<String>,
    port: Option<u32>,
    rasmus_email: String,
    sendgrid_api_key: String,
    sendgrid_api_url: String,
    from_email: String,
}

#[derive(Deserialize)]
pub struct RasmusRequest {
    question: String,
}

fn rasmus(config: Config, data: Form<RasmusRequest>) -> impl Responder {
    let client = reqwest::Client::new();

    match client
        .post(&config.sendgrid_api_url)
        .header(
            AUTHORIZATION,
            format!("Bearer {}", &config.sendgrid_api_key),
        )
        .json(&json!({
            "personalizations": [{"to": [{"email": &config.rasmus_email}]}],
            "from": {"email": &config.from_email},
            "subject": "Question",
            "content": [{"type": "text/plain", "value": data.question}]
        }))
        .send()
    {
        Ok(_) => {
            info!("Successfully forwarded a question to Rasmus");

            HttpResponse::Ok()
                .content_type("text/plain; charset=utf-8")
                .body(format!(
                    "Du har sendt et spørgsmål til Rasmus og Monopolet."
                ))
        }
        Err(response) => {
            error!("Couldn't send e-mail: {}", response);

            HttpResponse::BadGateway()
                .content_type("text/plain; charset=utf-8")
                .body(format!(
                    "Kunne ikke sende dit spørgsmål... Gå tilbage og prøv igen."
                ))
        }
    }
}

pub fn run(config: Config) {
    let config_ = config.clone();
    server::new(move || {
        let config_ = config_.clone();

        App::new().route("/", http::Method::POST, move |r| rasmus(config_.clone(), r))
    })
    .bind(format!(
        "{}:{}",
        config.host.unwrap_or("localhost".to_string()),
        config.port.unwrap_or(3000)
    ))
    .unwrap()
    .run();
}
