use diesel::prelude::*;
use serde_derive::Serialize;
use chrono::NaiveDateTime;


#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = super::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub premium: bool,
    pub suspended: bool,
}

// Account struct with sensible fields hidden from the user
#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = super::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FilteredAccount {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub premium: bool,
    pub suspended: bool,
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = super::schema::account_stats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountStats {
    pub id: i32,
    pub account_id: i32,
    pub first_log: NaiveDateTime,
    pub last_log: NaiveDateTime,
    pub games_played: i64,
    pub games_won: i64,
    pub wallet: i64,
    pub experience: i64,
    pub level: i32,
    pub season_rank: i32,
    pub best_rank: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = super::schema::friends)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Friend {
    pub id: i32,
    pub account1: i32,
    pub account2: i32,
    pub status: i32,
}