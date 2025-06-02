use std::time;
use std::env;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_governor::{Governor, GovernorConfigBuilder};
use sqlx::mysql::MySqlPoolOptions;
use actix_web::cookie::Key;
use actix_session::SessionMiddleware;
use actix_session::storage::SessionStore;
use serde::Deserialize;
use uuid::Uuid;

mod handlers;


// <!DOCTYPE html>
//         <html>
//         <head>
//             <title>Actix Web Example</title>
//             <script>
//                 async function updateElement() {
//                     const response = await fetch('/update');
//                     const data = await response.text();
//                     document.getElementById('myElement').innerText = data;
//                 }
//             </script>
//         </head>
//         <body>
//             <h1 id="myElement">Original Text</h1>
//             <button onclick="updateElement()">Update Text</button>
//         </body>
//         </html>

// #[get("/{name}")]
// async fn hello(name: web::Path<String>) -> impl Responder {
//     format!("Hello {}!", &name)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(2)
        .burst_size(30)
        .finish()
        .unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(time::Duration::from_secs(10))
        .connect("mysql://admindev:Sans154125@127.0.0.1:3306/DB_RID")
        .await
        .expect("pool failed");

    HttpServer::new(move || App::new()
        .wrap(Governor::new(&governor_conf))
        .app_data(web::Data::new(pool.clone()))
        .service(web::resource("/").route(web::get().to(handlers::main_page)))
        .service(web::resource("/find").route(web::post().to(handlers::find)))
        .service(web::resource("/best").route(web::get().to(handlers::best)))
        .service(web::resource("/counters").route(web::get().to(handlers::counters)))
        .service(web::resource("/{name}").route(web::get().to(handlers::index)))
        .service(web::resource("{name}/assets/content.css").route(web::get().to(handlers::styles)))
        .service(web::resource("/assets/{name}.js").route(web::get().to(handlers::scripts)))
        .service(web::resource("/assets/{file}.png").route(web::get().to(handlers::png)))
        .service(web::resource("/assets/{file}.svg").route(web::get().to(handlers::svg)))
        .service(web::resource("/assets/{file}.ttf").route(web::get().to(handlers::fonts)))
        .service(web::resource("/{name}/update").route(web::get().to(handlers::update)))
        .service(web::resource("/{name}/result").route(web::post().to(handlers::poster)))
        .default_service(web::route().to(handlers::not_found))
    )
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}

// use actix_web::{web, App, HttpServer, HttpResponse, Responder, cookie::{Cookie, CookieJar}, HttpRequest};
// use uuid::Uuid;

// async fn set_uuid_cookie() -> impl Responder {
//     // Generate a new UUID
//     let uuid = Uuid::new_v4();

//     // Create a cookie with the UUID
//     let cookie = Cookie::build("user_uuid", uuid.to_string())
//         .path("/") // Set the path for the cookie
//         .http_only(true) // Optional: makes the cookie inaccessible to JavaScript
//         .finish();

//     // Create a response and add the cookie
//     HttpResponse::Ok()
//         .cookie(cookie) // Add the cookie to the response
//         .body(format!("UUID generated and stored in cookie: {}", uuid))
// }

// async fn read_uuid_cookie(req: HttpRequest) -> impl Responder {
//     // Attempt to retrieve the cookie
//     if let Some(cookie) = req.cookie("user_uuid") {
//         // If the cookie exists, return its value
//         HttpResponse::Ok().body(format!("UUID from cookie: {}", cookie.value()))
//     } else {
//         // If the cookie does not exist, return a message
//         HttpResponse::Ok().body("No UUID cookie found.")
//     }
// }
