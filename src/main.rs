// //Calculator Microservice
// use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("This is a calculator microservice")
// }

// //library add route using lib.rs
// #[get("/add/{a}/{b}")]
// async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
//     let result = calc::add(info.0, info.1);
//     HttpResponse::Ok().body(result.to_string())
// }

// //library subtract route using lib.rs
// #[get("/subtract/{a}/{b}")]
// async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
//     let result = calc::subtract(info.0, info.1);
//     HttpResponse::Ok().body(result.to_string())
// }

// //library multiply route using lib.rs
// #[get("/multiply/{a}/{b}")]
// async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
//     let result = calc::multiply(info.0, info.1);
//     HttpResponse::Ok().body(result.to_string())
// }

// //library divide route using lib.rs
// #[get("/divide/{a}/{b}")]
// async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
//     let result = calc::divide(info.0, info.1);
//     HttpResponse::Ok().body(result.to_string())
// }

// //run it
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(index)
//             .service(add)
//             .service(subtract)
//             .service(multiply)
//             .service(divide)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        "This is a Chinese Zodiac microservice.\n 
    If result is 0, your Chinese Zodiac is rat. \n 
    If result is 1, your Chinese Zodiac is ox.\n 
    If result is 2, your Chinese Zodiac is tiger.\n 
    If result is 3, your Chinese Zodiac is rabbit.\n 
    If result is 4, your Chinese Zodiac is dragon.\n 
    If result is 5, your Chinese Zodiac is snake.\n 
    If result is 6, your Chinese Zodiac is horse.\n 
    If result is 7, your Chinese Zodiac is goat.\n 
    If result is 8, your Chinese Zodiac is monkey.\n 
    If result is 9, your Chinese Zodiac is rooster.\n 
    If result is 10, your Chinese Zodiac is dog.)\n 
    If result is 11, your Chinese Zodiac is pig.)",
    )
}

#[get("/cal/{a}/{b}")]
async fn cal(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::cal(info.0, info.1);
    HttpResponse::Ok().body(result.to_string())
}

//run it
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(cal))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
