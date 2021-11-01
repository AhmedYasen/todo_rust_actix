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
            .route(web::get().to(|| HttpResponse::Ok().body("update"))),
    )
    .service(
        web::resource("/delete")
            .route(web::get().to(|| HttpResponse::Ok().body("delete"))),
    )
    ;
}