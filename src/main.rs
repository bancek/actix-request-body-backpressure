use std::error::Error;
use std::{thread, time};

use futures::{future, Future, Stream};

use actix_web::{middleware, web, Error as ActixError, App, HttpResponse, HttpServer};

fn main() -> Result<(), Box<dyn Error>> {
    let listen = "127.0.0.1:8080";

    println!("Listening on {}", listen);

    Ok(
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .route("/", web::put().to_async(put_object))
        })
        .bind(&listen)?
        .run()?
    )
}

fn put_object(stream: web::Payload) -> impl Future<Item = HttpResponse, Error = ActixError> {
    stream
        .map_err(ActixError::from)
        .fold((), move |_, chunk| {
            web::block::<_, _, ()>(move || {
                thread::sleep(time::Duration::from_millis(1000));

                println!("Upload chunk {}", chunk.len());

                Ok(())
            })
        })
        .and_then(move |_| {
            future::ok(HttpResponse::Ok().body("Upload finished"))
        })
}
