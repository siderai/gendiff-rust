use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_json::{Value};
use json_diff::{diff, diff_config, GroupBy, Default};


async fn compare_json(data: web::Json<(Value, Value)>) -> impl Responder {
    let diff_config = diff_config {
        color: true,
        group_by: GroupBy::Type,
        ..Default::default()
    };

    let diff_result = diff(&data.0, &data.1, &diff_config);
    let diff_string = serde_json::to_string_pretty(&diff_result).unwrap_or_else(|_| "Unable to serialize diff result.".to_string());

    HttpResponse::Ok().body(diff_string)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/compare", web::post().to(compare_json))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
