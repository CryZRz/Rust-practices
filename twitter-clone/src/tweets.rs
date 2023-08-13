use actix_web::{get, post, web::{Path, Data}, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::{RunQueryDsl, r2d2::{ConnectionManager, Pool}, PgConnection};
use uuid::Uuid;
use super::schema::tweets;
use serde_derive::{Serialize, Deserialize};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};

use crate::constans::APPLICATION_JSON;


#[derive(Queryable, Insertable, Serialize, Deserialize)]
struct Tweet {
    id: Uuid,
    created_at: NaiveDateTime,
    message: String
}

impl Tweet{
    fn new(message: String) -> Tweet{
        Tweet { id: Uuid::new_v4(), created_at: Utc::now().naive_utc(), message }
    }
}

#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse{
    //obtener tweets
    use crate::schema::tweets::dsl::*;

    let mut conn = pool.get().expect("error al conectar con la db");
    let result = tweets.load::<Tweet>(&mut conn);

    let response = match result{
        Ok(tws) => tws,
        Err(_) => panic!("error")
    };

    HttpResponse::Ok().content_type(APPLICATION_JSON)
    .json(&response)
}

#[post("/tweets")]
pub async fn create_tweets(req_body: String, pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse{
    //obtener tweets
    let new_tweet = Tweet::new(req_body);

    let mut conn = pool.get().expect("error al conectar con la db");
    diesel::insert_into(tweets::table)
    .values(&new_tweet)
    .execute(&mut conn).expect("err to inset");

    HttpResponse::Created().content_type(APPLICATION_JSON)
    .json(&new_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet(path: Path<(String,)>) -> HttpResponse{
    //obtener tweets
    let tweet = format!("este es el tweet {}", path.0);
    HttpResponse::Ok().content_type(APPLICATION_JSON)
    .json(tweet)
}