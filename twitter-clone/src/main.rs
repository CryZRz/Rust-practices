//tweets -> GET: se obtien los tweets POST -> se crea un tweet
//tweets/:id -> GET: se obtien un tweet DELETE -> se borra un tweet
//tweets/:id/likes -> GET: se obtien los de likes de un tweet DELETE -> se borra un tweet

#[macro_use]
extern crate diesel;

use std::env;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use actix_web::{web::{self,}, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
mod constans;
mod schema;

mod likes;
mod tweets;

use likes::*;
use tweets::*;
use uuid::Uuid;

async fn saludar() -> impl Responder {
    HttpResponse::Ok().body("Zaida")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Like::new(Uuid::new_v4());
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("db url not found");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/", web::get().to(saludar))
            .service(get_tweets)
            .service(create_tweets)
            .service(get_tweet)
            .service(get_likes_tweet)
            .service(make_like)
            .service(remove_like)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
