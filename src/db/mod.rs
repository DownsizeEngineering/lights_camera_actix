use actix::prelude::*;
use actix_postgres::{PostgresActor, PostgresMessage};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use actix_postgres::bb8_postgres::tokio_postgres::{tls::NoTls, row::Row};

pub mod todo;

#[derive(Deserialize)]
struct DBCredentials {
    address: String,
    name: String,
    password: String,
}

pub type PGA = Addr<PostgresActor<NoTls>>;

pub fn get_actor() -> PGA {
    let db_cred: DBCredentials = serde_json::from_str(
        &std::fs::read_to_string("./secrets.json").unwrap()
    ).unwrap();

    let db_url = format!("postgresql://{}:{}@{}", 
        db_cred.name, db_cred.password, db_cred.address);

    PostgresActor::start(&db_url, NoTls).unwrap() 
}

async fn pg_query (db: &PGA, query: &str) -> Vec<Row> {
    let q = String::from(query);
    let task: PostgresMessage<_, NoTls, _> = PostgresMessage::new(|pool| {
        Box::pin(async move {
            let connection = pool.get().await?;
            connection
                .query(q.as_str(), &vec![])
                .await
                .map_err(|err| err.into())
        })
    });
    let res = db.send(task).await.unwrap().unwrap();
    res
}


struct User {
    name: String,
    id: i32,
}

pub async fn names(db: web::Data<PGA>) -> impl Responder {
    let res = pg_query(db.get_ref(), "SELECT * FROM names").await;

    let val= User {
        name: res[0].get(0),
        id: res[0].get(1),
    };

    HttpResponse::Ok().body(format!("{} {}", val.name, val.id))
}