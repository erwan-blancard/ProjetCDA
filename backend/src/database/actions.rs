use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::dsl::insert_into;
use diesel::PgConnection;
use diesel::prelude::*;
use serde_derive::Deserialize;

use crate::database::models::*;
use crate::database::schema::*;

use super::models::{Account, FilteredAccount, Friend};

#[derive(Insertable, Deserialize)]
#[diesel(table_name = super::schema::accounts)]
pub struct NewAccount {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AccountLogin {
    pub username: String,
    pub password: String,
}

pub fn create_account(
    conn: &mut PgConnection,
    username: &String,
    email: &String,
    password: &String,
) -> diesel::QueryResult<FilteredAccount> {
    use super::schema::accounts::dsl::{accounts, id};
    use super::schema::account_stats::dsl::account_stats;

    let hashed = hash(password, DEFAULT_COST).unwrap();

    let new_account = NewAccount {
        username: username.clone(),
        email: email.clone(),
        password: hashed,
    };

    conn.transaction(|conn| {
        insert_into(accounts).values(&new_account).execute(conn)?;
        let account = accounts
            .select(FilteredAccount::as_select())
            .order_by(id.desc())
            .first(conn)?;

        insert_into(account_stats)
            .values(NewEmptyStats { account_id: account.id })
            .execute(conn)?;
        Ok(account)
    })
}

pub fn get_account_for_login(
    conn: &mut PgConnection,
    username_in: &String,
    password_in: &String,
) -> diesel::QueryResult<FilteredAccount> {
    use super::schema::accounts::dsl::*;

    let acct: Account = accounts
        .filter(username.eq(username_in))
        .first(conn)?;

    if verify(password_in, &acct.password).unwrap_or(false) {
        accounts
            .select(FilteredAccount::as_select())
            .filter(id.eq(acct.id))
            .first(conn)
    } else {
        Err(diesel::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bcrypt::verify;
    use diesel::connection::Connection;
    use diesel::PgConnection;
    use dotenv::dotenv;
    use std::env;
    use crate::database::schema::accounts::dsl::*;
    use crate::database::models::Account;

    fn get_test_conn() -> PgConnection {
        dotenv().ok();
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&url).unwrap()
    }

    #[test]
    fn test_create_account_hashes_password() {
        let mut conn = get_test_conn();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let user = "foo".to_string();
            let mail = "foo@local".to_string();
            let pass = "password123".to_string();

            let filtered = create_account(&mut conn, &user, &mail, &pass)?;
            assert_eq!(filtered.username, user);

            let acct: Account = accounts
                .filter(id.eq(filtered.id))
                .first(&mut conn)?;
            assert_ne!(acct.password, pass);
            assert!(verify(&pass, &acct.password).unwrap());
            Ok(())
        });
    }

    #[test]
    fn test_get_account_for_login() {
        let mut conn = get_test_conn();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let user = "bar".to_string();
            let mail = "bar@local".to_string();
            let pass = "secret!".to_string();

            let _ = create_account(&mut conn, &user, &mail, &pass)?;
            let ok = get_account_for_login(&mut conn, &user, &pass)?;
            assert_eq!(ok.username, user);

            let err = get_account_for_login(&mut conn, &user, &"nop".to_string());
            assert!(err.is_err());
            Ok(())
        });
    }
}