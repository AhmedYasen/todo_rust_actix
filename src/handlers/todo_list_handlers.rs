use actix_web::{web, HttpResponse};
use crate::models::todo_list::TodoList;
use crate::models::todo_item::TodoItem;
use serde::{Deserialize, Serialize};


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

#[derive(Serialize)]
struct FullList {
    list: TodoList,
    items: Option<Vec<TodoItem>>,
}
async fn read_list(req: web::Path<ListInfo>) -> HttpResponse {
    let list = TodoList::read(req.0.list_id);

    
    if let Err(err) = list {
        return HttpResponse::InternalServerError().body(err.to_string());
    }
    let list = list.unwrap();
    let items = TodoItem::read_all_by_list_id(list.id);
    
    HttpResponse::Created().json(FullList {list, items,})
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
        TodoItem::delete_all_by_list_id(req.list_id);
        TodoList::delete(req.list_id);
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
            .route(web::delete().to(delete_list)),
    )
    ;
}