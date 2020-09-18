use std::{thread, time};

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    let listen = "127.0.0.1:8080";

    println!("Listening on {}", listen);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::put().to(put_object))
    })
    .bind(&listen)?
    .run()
    .await
}

async fn put_object(_payload: web::Payload) -> HttpResponse {
    web::block::<_, _, ()>(move || {
        thread::sleep(time::Duration::from_millis(10000));
        Ok(())
    }).await.unwrap();

    HttpResponse::Ok().body("Upload finished")
}
