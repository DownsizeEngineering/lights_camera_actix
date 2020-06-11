use actix::prelude::*;
use actix_postgres::{PostgresActor, PostgresMessage};
use actix_web::{HttpResponse, Responder};
use serde::Deserialize;
use actix_postgres::bb8_postgres::tokio_postgres::{tls::NoTls, row::Row};

#[derive(Deserialize)]
struct DBCredentials {
    address: String,
    name: String,
    password: String,
}

pub fn get_actor() -> Addr<PostgresActor<NoTls>> {
    let db_cred: DBCredentials = serde_json::from_str(
        &std::fs::read_to_string("./secrets.json").unwrap()
    ).unwrap();

    let db_url = format!("postgresql://{}:{}@{}", 
        db_cred.name, db_cred.password, db_cred.address);

    PostgresActor::start(&db_url, NoTls).unwrap() 
}

async fn pg_query (query: &str) -> Vec<Row> {
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
    let res = get_actor().send(task).await.unwrap().unwrap();
    // let val:&str = res[0].get(0);
    // val.to_string()
    res
}


struct User {
    name: String,
    id: i32,
}

pub async fn names() -> impl Responder {
    let res = pg_query("SELECT * FROM names").await;

    let val= User {
        name: res[0].get(0),
        id: res[0].get(1),
    };

    HttpResponse::Ok().body(format!("{} {}", val.name, val.id))
}