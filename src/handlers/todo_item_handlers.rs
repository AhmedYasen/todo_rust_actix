use actix_web::{web, HttpResponse};

// this function could be located in a different module
pub fn todo_item_config(cfg: &mut web::ServiceConfig) {
    
    cfg
    .service(
        web::resource("/create")
            .route(web::post().to(|| HttpResponse::Ok().body("create"))),
    )
    .service(
        web::resource("/read")
            .route(web::get().to(|| HttpResponse::Ok().body("read"))),
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