use std::time;
use std::env;

use actix_web::{web, App, HttpServer};
use actix_governor::{Governor, GovernorConfigBuilder};
use sqlx::mysql::MySqlPoolOptions;
use actix_web::middleware::Logger;

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


#[actix_web::main]
async fn main() -> std::io::Result<()>
{
     let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")?;

    env_logger::Builder::from_env(env_logger::Env::new().filter("error"))
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(2)
        .burst_size(30)
        .finish()
        .unwrap();

    

    dotenvy::dotenv().ok();

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(time::Duration::from_secs(10))
        .connect(&(env::var("DATABASE_URL").expect("No env var found")))
        .await
        .expect("pool failed");

    HttpServer::new(move || App::new()
        .wrap(Logger::default())
        .wrap(Governor::new(&governor_conf))
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::new(env::var("FILE_PATH").expect("No env var found").clone()))
        .service(web::resource("/").route(web::get().to(handlers::main_page)))
        .service(web::resource("/favicon.ico").route(web::get().to(handlers::favicon)))
        .service(web::resource("/find").route(web::post().to(handlers::find)))
        .service(web::resource("/best").route(web::get().to(handlers::best)))
        .service(web::resource("/counters").route(web::get().to(handlers::counters)))
        .service(web::resource("/nominated").route(web::get().to(handlers::nominated)))
        .service(web::resource("/{name}").route(web::get().to(handlers::index)))
        .service(web::resource("{name}/assets/content.css").route(web::get().to(handlers::styles)))
        .service(web::resource("/assets/{name}.js").route(web::get().to(handlers::scripts)))
        // .service(web::resource("/assets/{file}.png").route(web::get().to(handlers::png)))
        // .service(web::resource("/assets/{file}.svg").route(web::get().to(handlers::svg)))
        // .service(web::resource("/assets/{file}.ttf").route(web::get().to(handlers::fonts)))
        // .service(web::resource("/{name}/update").route(web::get().to(handlers::update)))
        // .service(web::resource("/{name}/result").route(web::post().to(handlers::poster)))
        .default_service(web::route().to(handlers::not_found))
    )
        .bind(("0.0.0.0", 81))?
        .run()
        .await
}
