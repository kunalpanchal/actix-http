use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("actix homepage !")
}

#[post("/add-music")]
async fn add_music(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Music is added now: ".to_owned() + &req_body.to_owned())
}

async fn get_music() -> impl Responder {
    HttpResponse::Ok().body("There you go, have some music.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(add_music)
            .route("/get-music", web::get().to(get_music))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
