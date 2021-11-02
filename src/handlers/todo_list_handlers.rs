use actix_web::{web, HttpResponse};
use crate::models::todo_list::TodoList;
use serde::Deserialize;


async fn create_list(req: web::Json<TodoList>) -> HttpResponse {
    

    let mut list = req.0.clone();
    if let Some(err) = list.create() {
        return HttpResponse::InternalServerError().body(err.to_string());
    }

    HttpResponse::Created().json(list)
    
}

#[derive(Deserialize)]
struct ListInfo {
    list_id: u16,
}
async fn read_list(req: web::Path<ListInfo>) -> HttpResponse {
    let list = TodoList::read(req.0.list_id);

    
    if let Err(err) = list {
        return HttpResponse::InternalServerError().body(err.to_string());
    }

    HttpResponse::Created().json(list.unwrap())
}

async fn update_list(req: web::Json<TodoList>) -> HttpResponse {
    let mut list = req.0.clone();
    if let Some(err) = list.update() {
        return HttpResponse::InternalServerError().body(err.to_string());
    }

    HttpResponse::Ok().json(list)
}

async fn delete_list(req: web::Path<ListInfo>) -> HttpResponse {

    //check if the list is found
    if TodoList::is_exist(req.list_id) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().body("This list is not found")
    }
}


pub fn todo_list_config(cfg: &mut web::ServiceConfig) {
    
    cfg
    .service(
        web::resource("/create")
            .route(web::post().to(create_list)),
    )
    .service(
        web::resource("/read/{list_id}")
            .route(web::get().to(read_list)),
    )
    .service(
        web::resource("/update")
            .route(web::put().to(update_list)),
    )
    .service(
        web::resource("/delete/{list_id}")
            .route(web::get().to(delete_list)),
    )
    ;
}