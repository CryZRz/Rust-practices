use std::str::FromStr;

use actix_web::{get, post, web::{Path, Data}, HttpResponse, delete};
use chrono::{NaiveDateTime, Utc};
use diesel::{RunQueryDsl, r2d2::{ConnectionManager, Pool}, PgConnection};
use super::schema::likes;
use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};

use crate::constans::APPLICATION_JSON;


#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Like {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub tweet_id: Uuid,
}

impl Like {
    pub fn new(tweet_id: Uuid) -> Self{
        Self{
            id: Uuid::new_v4(), 
            created_at: Utc::now().naive_utc(),
            tweet_id
        }
    }
}

#[get("/tweets/{id}/likes")]
pub async fn get_likes_tweet(path: Path<(String,)>) -> HttpResponse{
    //obtener tweets
    let likes = 100;
    HttpResponse::Ok().content_type(APPLICATION_JSON)
    .json(likes)
}

#[post("/tweets/{id}/likes")]
pub async fn make_like(path: Path<(String,)>, pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse{
    //obtener tweets
    use crate::schema::likes::dsl::*;

    println!("{}", path);
    let t_id = &path.0;
    println!("hola");
    let mut conn = pool.get().expect("error al conectar con la db");

    let like: Like = Like::new(Uuid::from_str(t_id).unwrap());

    diesel::insert_into(likes).values(&like).execute(&mut conn).unwrap();
    HttpResponse::Created().content_type(APPLICATION_JSON)
    .json(&like)
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_like(path: Path<(String,)>) -> HttpResponse{
    //obtener tweets

    HttpResponse::NoContent().content_type(APPLICATION_JSON)
    .await
    .unwrap()
}