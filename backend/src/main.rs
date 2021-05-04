use actix_web::{get, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Game {
    id: String,
    name: String,
    icon: String,
}

#[get("/games")]
async fn games() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json([
        Game {
            id: "id1".to_string(),
            name: "Dune (2019)".to_string(),
            icon: "https://tesera.ru/images/items/1516691,3/200x200xpa/photo1.jpg".to_string(),
        },
        Game {
            id: "id2".to_string(),
            name: "Dominion".to_string(),
            icon: "https://tesera.ru/images/items/2496,3/200x200xca/photo1.jpg".to_string(),
        },
        Game {
            id: "id3".to_string(),
            name: "Nexus Ops (FFG 2011)".to_string(),
            icon: "https://tesera.ru/images/items/66318,3/200x200xpa/photo1.jpg".to_string(),
        },
    ]))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(games)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
