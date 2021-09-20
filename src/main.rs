use actix_web::{rt::System, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
use sqlx::SqlitePool;
use std::sync::mpsc;
use std::thread;

mod model;
use crate::model::Asset;
mod templates;
use crate::templates::index::IndexTemplate;

const SQLITE_URL: &str = "sqlite://snackotron.db";

async fn get(pool: web::Data<SqlitePool>) -> impl Responder {
    let assets = Asset::get_all(&pool).await;
    match assets {
        //Ok(a) => HttpResponse::Ok().json(a),
        Ok(a) => {
            let index = IndexTemplate { assets: &a };
            HttpResponse::Ok().content_type("text/html").body(index.render().unwrap())
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn db(_req: HttpRequest, pool: web::Data<SqlitePool>) -> impl Responder {
    let asset = model::Asset {
        upc: _req.match_info().get("upc").unwrap().parse::<i64>().unwrap(),
        count: _req.match_info().get("count").unwrap().parse::<i32>().unwrap(),
        unit: _req.match_info().get("unit").unwrap().to_string(),
        common_name: _req.match_info().get("common_name").unwrap().to_string()
    };
    match asset.register(&pool).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn get_beans(pool: web::Data<SqlitePool>) -> impl Responder {
    let asset = Asset::get(&pool).await;
    match asset {
        Ok(a) => HttpResponse::Ok().json(a),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() {
    let pool = SqlitePool::connect(SQLITE_URL).await.unwrap();

    {
        let (tx, _ /*rx*/) = mpsc::channel();

        let server_thread = thread::spawn(move || {
            let sys = System::new("http-server");

            let srv = HttpServer::new(move || {
                App::new().data(pool.clone())
                    .route("/", web::get().to(get))
                    .route("/add/{upc}/{count}/{unit}/{common_name}", web::get().to(db))
                    .route("/gimmeBeans", web::get().to(get_beans))
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
}
