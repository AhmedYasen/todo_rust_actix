use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::models::todo_item::TodoItem;


async fn create_item(item: web::Json<TodoItem>) -> HttpResponse {
    let mut item: TodoItem = item.0.clone();

    if let Some(err) = item.create() {
        return HttpResponse::InternalServerError().body(err.to_string());
    }

    HttpResponse::Created().json(item)
}

#[derive(Deserialize)]
struct ItemInfo {
    item_id: u16,
}
async fn read_item(item: web::Path<ItemInfo>) -> HttpResponse {
    let item = TodoItem::read(item.0.item_id);

    if let Err(err) = item {
        return HttpResponse::InternalServerError().body(err.to_string());
    }

    HttpResponse::Ok().json(item.unwrap())
}

async fn update_item(item: web::Json<TodoItem>) -> HttpResponse {
    let mut item: TodoItem = item.0.clone();

    if let Some(err) = item.update() {
        return HttpResponse::InternalServerError().body(err.to_string());
    }

    HttpResponse::Ok().json(item)
}

async fn delete_item(item: web::Path<ItemInfo>) -> HttpResponse {
    TodoItem::delete(item.0.item_id);
    HttpResponse::Ok().finish()

}

pub fn todo_item_config(cfg: &mut web::ServiceConfig) {
    
    cfg
    .service(
        web::resource("/create")
            .route(web::post().to(create_item)),
    )
    .service(
        web::resource("/read/{item_id}")
            .route(web::get().to(read_item)),
    )
    .service(
        web::resource("/update")
            .route(web::put().to(update_item)),
    )
    .service(
        web::resource("/delete/{item_id}")
            .route(web::delete().to(delete_item)),
    )
    ;
}