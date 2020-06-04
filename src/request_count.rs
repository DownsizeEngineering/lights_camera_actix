use super::*;

async fn request_count(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request #{}", counter)
}

pub fn config_request_count(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/requests")
        .route(web::get().to(request_count)),
    );
}