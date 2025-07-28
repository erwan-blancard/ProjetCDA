pub mod auth;
pub mod database;
pub mod routes;
pub mod server;
pub mod utils;
pub mod dto;

pub use database::actions;
pub use database::models;
pub use database::schema;

pub type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>; 