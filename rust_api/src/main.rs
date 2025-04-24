use actix_web::{get,post,web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info{
    value: String,
}

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("we made it!")
}

#[post("/echo")]
async fn post_fn(query: web::Query<Info>) -> impl Responder{
    HttpResponse::Ok().body(format!("{}",query.value))

}

async fn manual_route() -> impl Responder{
    HttpResponse::Ok().body("This is manual routing without macros??")
}


#[actix_web::main]
async fn main()-> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(post_fn)
        .route("/manual", web::get().to(manual_route))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(post_fn)
//             .route("/hey", web::get().to(manual_route))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }