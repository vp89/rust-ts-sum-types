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
        .connect("postgres://postgres:postgres@localhost:5432/animals")
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
            .service(get_animal_by_id)
            .service(insert_animal_by_id)
    })
    .listen(listener)?
    .run()
    .await?)
}

#[derive(Debug, Deserialize, Serialize)]
struct Animal {
    animal_id: i32,
    animal: Json<AnimalKind>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum AnimalKind {
    Dog { a: String, b: String },
    Cat { c: String, d: String },
    Chicken { a: String, b: String, c: Option<String> }
}

impl AnimalKind {
    fn kind_id(&self) -> i32 {
        match self {
            AnimalKind::Dog { a, b } => 1,
            AnimalKind::Cat { c, d } => 2,
            AnimalKind::Chicken { a, b, c } => 3,
        }
    }
}

#[get("/")]
pub async fn get_animal_by_id(pool: web::Data<Pool<Postgres>>) -> actix_web::Result<HttpResponse> {
    let animal = sqlx::query_as!(
        Animal,
        r#"
        SELECT animal_id, animal as "animal: Json<AnimalKind>" FROM animals WHERE animal_id = 2;
        "#)
    .fetch_one(&**pool)
    .await;

    match animal {
        Ok(a) => Ok(HttpResponse::Ok().json(a)),
        Err(e) => {
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
    
        }
    }
}

#[get("/insert")]
pub async fn insert_animal_by_id(pool: web::Data<Pool<Postgres>>) -> actix_web::Result<HttpResponse> {
    let dog = AnimalKind::Dog { a: "foobar".to_string(), b: "barbaz".to_string() };
    
    let result = sqlx::query!(
        r#"INSERT INTO animals (animal, animal_kind_id) VALUES ( $1, $2 )"#,
        Json(&dog) as _,
        &dog.kind_id()
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
