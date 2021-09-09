use std::io::{Error};
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use sqlx::types::Json;
use serde::{Deserialize, Serialize};
use actix_web::{get, web, App, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:postgres@localhost:5432/content_catalog")
        .await
        .map_err(|err| {
            std::io::Error::new(std::io::ErrorKind::Other, err)
        })?;
    let data = web::Data::new(pool);

    let listen_addr = format!("127.0.0.1:4000");
    let listener = TcpListener::bind(listen_addr)
        .map_err(|err| {
            std::io::Error::new(std::io::ErrorKind::Other, err)
        })?;

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(get_content_entry_by_id)
            .service(insert_content_entry)
    })
    .listen(listener)?
    .run()
    .await?)
}

#[derive(Debug, Deserialize, Serialize)]
struct ContentEntry {
    entry_id: i32,
    content_url: String,
    content_metadata: Json<ContentMetadata>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum ContentMetadata {
    Music { artist: String, label: String, genre: String, tracks: i32 },
    Television { director: String, producer: String, seasons: i32, episodes: i32 },
    Film { director: String, producer: String, duration_mins: i32 }
}

impl ContentMetadata {
    fn content_type_id(&self) -> i32 {
        match self {
            ContentMetadata::Music { .. } => 1,
            ContentMetadata::Television { .. }=> 2,
            ContentMetadata::Film { .. }=> 3
        }
    }
}

#[get("/")]
pub async fn get_content_entry_by_id(pool: web::Data<Pool<Postgres>>) -> actix_web::Result<HttpResponse> {
    let content_entry = sqlx::query_as!(
        ContentEntry,
        r#"
        SELECT entry_id, content_url, content_metadata as "content_metadata: Json<ContentMetadata>"
        FROM content_entries
        WHERE entry_id = 1;
        "#)
    .fetch_one(&**pool)
    .await;

    match content_entry {
        Ok(a) => Ok(HttpResponse::Ok().json(a)),
        Err(e) => {
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/insert")]
pub async fn insert_content_entry(pool: web::Data<Pool<Postgres>>) -> actix_web::Result<HttpResponse> {
    let content_entry = ContentMetadata::Film {
        director: "Steven Spielberg".to_string(),
        producer: "Mr Foobar".to_string(),
        duration_mins: 123,
    };
    
    let result = sqlx::query!(
        r#"
        INSERT INTO content_entries (content_metadata, content_type_id, content_url)
        VALUES ( $1, $2, $3 )"#,
        Json(&content_entry) as _,
        &content_entry.content_type_id(),
        "http://google.com"
    )
    .execute(&**pool)
    .await;

    match result {
        Ok(a) => Ok(HttpResponse::Ok().finish()),
        Err(e) => {
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
