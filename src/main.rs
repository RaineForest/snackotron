use actix_web::{rt::System, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
use dotenv::dotenv;
use std::sync::mpsc;
use std::thread;

mod model;
use crate::model::*;
mod templates;
use crate::templates::index::IndexTemplate;
mod db_constants;
use db_constants::*;
mod upc;

#[derive(Clone)]
struct Context {
    pool: DBPool,
    upc_api: upc::UpcApi
}

async fn get(ctx: web::Data<Context>) -> impl Responder {
    let assets = Pantry::get_all(&ctx.pool).await;
    match assets {
        //Ok(a) => HttpResponse::Ok().json(a),
        Ok(a) => {
            let index = IndexTemplate { assets: &a };
            HttpResponse::Ok().content_type("text/html").body(index.render().unwrap())
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn add(_req: HttpRequest, ctx: web::Data<Context>) -> impl Responder {

    let obj = ctx.upc_api.lookup(_req.match_info().get("upc").unwrap());

    
    HttpResponse::NotImplemented()
}

async fn db(_req: HttpRequest, ctx: web::Data<Context>) -> impl Responder {
    let asset = model::Pantry {
        upc: _req.match_info().get("upc").unwrap().parse::<i64>().unwrap(),
        amount: _req.match_info().get("count").unwrap().parse::<f32>().unwrap(),
        unit: _req.match_info().get("unit").unwrap().to_string(),
        package_type: Package::Whole,
        brand: "Ajax".to_string()
    };
    match asset.register(&ctx.pool).await {
        Ok(_) => HttpResponse::Ok().json(()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() {
    dotenv().ok();

    let ctx = Context {
        pool: DBPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap(),
        upc_api: upc::UpcApi::new(std::env::var("UPC_TOKEN").unwrap())
    };

    let (tx, _ /*rx*/) = mpsc::channel();

    let server_thread = thread::spawn(move || {
        let sys = System::new("http-server");

        let srv = HttpServer::new(move || {
            App::new().data(ctx.clone())
                .route("/", web::get().to(get))
                .route("/add/{upc}", web::get().to(add))
                .route("/add/{upc}/{count}/{unit}", web::get().to(db))
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
