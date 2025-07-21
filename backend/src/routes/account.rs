use actix_web::{error, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_web::get;

use crate::{database::actions, DbPool};


#[get("/account/profile/{account_id}")]
async fn get_other_account(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
    let (account_id,) = path.into_inner();

    let account = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account_by_id(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(account))
}

#[get("/account/profile")]
async fn get_my_account(req: HttpRequest, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    // get account id based on JWT (put in extensions by JwtMiddleware)
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let account = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account_by_id(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(account))
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_my_account)
        .service(get_other_account);
}
