use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod availabilities;
use availabilities::{get_availabilities, NoAvailabilityReason};

#[get("/")]
async fn hello() -> String {
    // HttpResponse::Ok().body("Hello world!")
    String::from("Hello world!")
    // NoAvailabilityReason::BankHoliday
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> String {
    // HttpResponse::Ok().body("Hey there!")
    // NoAvailabilityReason::BankHoliday
    String::from("asdf")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_availabilities)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[actix_web::test]
    async fn test_something_async() {
        assert_eq!(manual_hello().await, "asdf")
    }
}
