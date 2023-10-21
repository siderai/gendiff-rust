use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_json::Value;
use difference::Changeset;

fn compare_json(json1: &(Value, Value), json2: &Value) -> String {
    let json_string1 = serde_json::to_string_pretty(&json1).unwrap();
    let json_string2 = serde_json::to_string_pretty(&json2).unwrap();

    let changeset = Changeset::new(&json_string1, &json_string2, "\n");
    changeset.to_string()
}

// Define a handler for the compare endpoint
async fn compare_json_handler(data: web::Json<(Value, Value)>) -> impl Responder {
    let comparison_result = compare_json(&data.0, &data.1);
    HttpResponse::Ok().body(comparison_result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/compare", web::post().to(compare_json_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
