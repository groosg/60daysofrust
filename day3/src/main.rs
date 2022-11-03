use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use rand::Rng;
use std::cmp::Ordering;

struct AppState {
    secret_number: u32,
}

async fn guess(guess_request: String, data:web::Data<AppState>) -> impl Responder {
    let secret_number = &data.secret_number;

    let guess: u32 = match guess_request.trim().parse() {
        Ok(num) => num,
        Err(msg) => {
            return HttpResponse::BadRequest().body(format!("{msg}"));
        }
    };

    let gues_response = match guess.cmp(secret_number) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    };

    HttpResponse::Ok().body(format!("{gues_response}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                secret_number,
            }))
            .route("/guess", web::post().to(guess))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

