use super::*;

#[derive(Deserialize)]
struct DBCredentials {
    address: String,
    name: String,
    password: String,
}

async fn pg () -> String{
    let db_cred: DBCredentials = serde_json::from_str(
        &std::fs::read_to_string("./secrets.json").unwrap()
    ).unwrap();

    let db_url = format!("postgresql://{}:{}@{}", 
        db_cred.name, db_cred.password, db_cred.address);

    let pg_actor = PostgresActor::start(&db_url, NoTls).unwrap(); 
    let task: PostgresMessage<_, NoTls, _> = PostgresMessage::new(|pool| {
        Box::pin(async move {
            let connection = pool.get().await?;
            connection
                .query("SELECT * FROM names", &vec![])
                .await
                .map_err(|err| err.into())
        })
    });
    let res = pg_actor.send(task).await.unwrap().unwrap();
    let val:&str = res[0].get(0);
    val.to_string()
}

pub async fn names() -> impl Responder {
    let val = pg().await;
    HttpResponse::Ok().body(val)
}