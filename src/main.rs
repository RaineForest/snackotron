use actix_web::{rt::System, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::convert::TryInto;
use std::sync::mpsc;
use std::thread;

mod model;
use model::Asset;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn db() -> impl Responder {
    let asset = model::Asset {
        upc: 0,
        count: 1,
        unit: String::from("beans"),
        common_name: String::from("beans")
    };
    match asset.register().await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn GETbEANS() -> impl Responder {
    let asset = Asset::get().await;
    match asset {
        Ok(a) => HttpResponse::Ok().json(a),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() {
    let (tx, _ /*rx*/) = mpsc::channel();

    let server_thread = thread::spawn(move || {
        let sys = System::new("http-server");

        let srv = HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(greet))
                .route("/hello/{name}", web::get().to(greet))
                .route("/db", web::get().to(db))
                .route("/gimmeBeans", web::get().to(GETbEANS))
        })
        .bind("127.0.0.1:8080")?
        .shutdown_timeout(60) // <- Set shutdown timeout to 60 seconds
        .run();

        let _ = tx.send(srv);
        sys.run()
    });

    //let srv = rx.recv().unwrap();

    // pause accepting new connections
    //srv.pause().await;
    // resume accepting new connections
    //srv.resume().await;
    // stop server
    //srv.stop(true).await;

    let _ = server_thread.join().unwrap();
}
