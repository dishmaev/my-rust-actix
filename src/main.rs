mod handlers;

use actix_web::{web, App, HttpServer};
use handlers::{event, index, index2, index3};
use std::env;
use std::io::Write;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const DEFAULT_HOST: &str = "127.0.0.1";
    const ENV_HOST: &str = "MY_BIN_HOST";
    const ENV_PORT: &str = "PORT";

    let host = env::var(ENV_HOST).unwrap_or(String::from(DEFAULT_HOST));
    match env::var_os(ENV_PORT) {
        Some(val) => {
            let port = val.into_string().unwrap().parse::<usize>().unwrap();
            HttpServer::new(|| {
                App::new()
                    .route("/", web::post().to(index))
                    .route("/2", web::post().to(index2))
                    .route("/3", web::post().to(index3))
                    .route("/event", web::post().to(event))
            })
            .bind(format!("{}:{}", host, port))?
            .run()
            .await
        }
        None => writeln!(
            std::io::stderr(),
            "{} is not defined in the environment.",
            ENV_PORT
        ),
    }
}
