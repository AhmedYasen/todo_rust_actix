mod settings;
mod handlers;
mod models;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let settings = settings::Settings::new().unwrap();
    
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/list").configure(handlers::todo_list_handlers::todo_list_config))
            .service(web::scope("/item").configure(handlers::todo_item_handlers::todo_item_config))
    })
    .bind(format!("{}:{}", settings.server_config.host, settings.server_config.port))?
    .run()
    .await
}