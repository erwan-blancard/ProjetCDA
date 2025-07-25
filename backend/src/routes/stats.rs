use actix_web::{error, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_web::get;

use crate::database::models::AccountStats;
use crate::{database::actions, DbPool};

#[utoipa::path(
    get,
    path = "/account/stats",
    responses(
        (status = 200, description = "Get stats for your account", body = AccountStats),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = [])),
    tag = "Stats"
)]
#[get("/account/stats")]
async fn get_my_account_stats(req: HttpRequest, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let stats = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account_stats(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(stats))
}

#[utoipa::path(
    get,
    path = "/account/stats/{account_id}",
    params(
        ("account_id" = i32, Path, description = "ID of the account to get stats for")
    ),
    responses(
        (status = 200, description = "Get stats of an account", body = AccountStats),
        (status = 500, description = "Internal server error")
    ),
    tag = "Stats"
)]
#[get("/account/stats/{account_id}")]
async fn get_other_account_stats(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
    let (account_id,) = path.into_inner();

    let stats = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account_stats(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(stats))
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_my_account_stats)
        .service(get_other_account_stats);
}
