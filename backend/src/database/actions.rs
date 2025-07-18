use chrono::NaiveDateTime;
use diesel::dsl::insert_into;
use diesel::PgConnection;
use diesel::prelude::*;
use serde_derive::{Serialize, Deserialize};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::database::models::*;
use crate::database::schema::*;

use super::models::{Account, Friend};

// DTOs

#[derive(Insertable, Deserialize)]
#[diesel(table_name = super::schema::accounts)]
pub struct NewAccount {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = super::schema::accounts)]
pub struct AccountLogin {
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = super::schema::friends)]
pub struct FriendRequest {
    pub account1: i32,
    pub account2: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = super::schema::account_stats)]
pub struct NewEmptyStats {
    pub account_id: i32,
}


pub fn create_account(conn: &mut PgConnection, username: &String, email: &String, password: &String) -> diesel::QueryResult<FilteredAccount> {
    use super::schema::accounts::dsl::{accounts, id};
    use super::schema::account_stats::dsl::account_stats;

    // hash password before storing
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");

    let new_account = NewAccount {
        username: username.to_string(),
        email: email.to_string(),
        password: hashed_password,
    };

    conn.transaction(|conn| {
        insert_into(accounts)
            .values(&new_account)
            .execute(conn)?;
        
        // get newly inserted account
        let account = accounts.select(FilteredAccount::as_select())
            .order_by(id.desc())
            .first(conn)?;

        // create stats entry
        insert_into(account_stats)
            .values(NewEmptyStats {account_id: account.id} )
            .execute(conn)?;
    
        Ok(account)
    })
}

pub fn get_account_by_id(conn: &mut PgConnection, account_id: i32) -> diesel::QueryResult<FilteredAccount> {
    use super::schema::accounts::dsl::*;

    let account = accounts.select(FilteredAccount::as_select())
        .filter(id.eq(&account_id))
        .first::<FilteredAccount>(conn)?;

    Ok(account)
}

pub fn get_accounts_by_id(conn: &mut PgConnection, account_ids: &Vec<i32>) -> diesel::QueryResult<Vec<FilteredAccount>> {
    use super::schema::accounts::dsl::*;

    let accs = accounts.select(FilteredAccount::as_select())
        .filter(id.eq_any(account_ids))
        .load(conn)?;

    Ok(accs)
}

pub fn get_account_by_username(conn: &mut PgConnection, username: &String) -> diesel::QueryResult<FilteredAccount> {
    accounts::table.select(FilteredAccount::as_select())
        .filter(accounts::dsl::username.eq(username))
        .first(conn)
}

pub fn get_full_account_by_email(conn: &mut PgConnection, email: &String) -> diesel::QueryResult<Account> {
    accounts::table.select(Account::as_select())
        .filter(accounts::dsl::email.eq(email))
        .first(conn)
}

pub fn get_account_for_login(conn: &mut PgConnection, username: &String, password: &String) -> diesel::QueryResult<FilteredAccount> {
    use super::schema::accounts::dsl;
    let account = dsl::accounts
        .filter(dsl::username.eq(username))
        .first::<Account>(conn)?;

    if verify(password, &account.password).unwrap_or(false) {
        Ok(FilteredAccount::from(account))
    } else {
        Err(diesel::result::Error::NotFound)
    }
}


pub fn get_account_stats(conn: &mut PgConnection, account_id: i32) -> diesel::QueryResult<AccountStats> {
    account_stats::table.select(AccountStats::as_select())
        .filter(account_stats::dsl::account_id.eq(account_id))
        .first(conn)
}


pub fn send_friend_request(conn: &mut PgConnection, sender_id: i32, username: &String) -> diesel::QueryResult<Friend> {
    use super::schema::friends::dsl::{friends, id};

    let target_account_id: i32 = accounts::table.select(accounts::id)
        .filter(accounts::dsl::username.eq(username))
        .first(conn)
        .expect("The account doesn't exist !");

    let friend_request = FriendRequest {
        account1: sender_id,
        account2: target_account_id
    };

    conn.transaction(|conn| {
        insert_into(friends)
            .values(&friend_request)
            .execute(conn)?;
        
        // get newly inserted friend
        let friend = friends.select(Friend::as_select())
            .order_by(id.desc())
            .first(conn)?;
    
        Ok(friend)
    })
}

pub fn get_friend_request_of_account_by_username(conn: &mut PgConnection, receiver_id: i32, username: &String) -> diesel::QueryResult<Friend> {
    use super::schema::friends::dsl::{friends, account1, account2, status};

    let target_account_id: i32 = accounts::table.select(accounts::id)
        .filter(accounts::dsl::username.eq(username))
        .first(conn)?;

    friends.select(Friend::as_select())
        .filter(
            account2.eq(receiver_id)
            .and(account1.eq(target_account_id))
            .and(status.eq(0))
        )
        .first(conn)
}

pub fn change_friend_request_status(conn: &mut PgConnection, receiver_id: i32, username: &String, accepted: bool) -> diesel::QueryResult<Friend> {
    use super::schema::friends::dsl::{friends, status};

    let new_status = if accepted { 1 } else { 2 };

    let request: Friend = get_friend_request_of_account_by_username(conn, receiver_id, username)?;

    conn.transaction(|conn| {
        diesel::update(friends.find(request.id))
            .set(status.eq(new_status))
            .execute(conn)?;

        let friend = friends.find(request.id)
            .select(Friend::as_select())
            .first(conn)?;

        Ok(friend)
    })
}

pub fn list_friends_for_account(conn: &mut PgConnection, account_id: i32) -> diesel::QueryResult<Vec<Friend>> {
    use super::schema::friends::dsl::{friends, account1, account2, status};

    friends.select(Friend::as_select())
        .filter(
            account1.eq(account_id)
            .or(account2.eq(account_id))
            .and(status.eq(1))
        )
        .load(conn)
}

pub fn list_friend_requests_for_account(conn: &mut PgConnection, account_id: i32) -> diesel::QueryResult<Vec<Friend>> {
    use super::schema::friends::dsl::{friends, account1, account2, status};

    friends.select(Friend::as_select())
        .filter(
            account1.eq(account_id)
            .or(account2.eq(account_id))
            .and(status.ne(1))
        )
        .load(conn)
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = super::schema::password_reset_tokens)]
pub struct NewPasswordResetToken {
    pub account_id: i32,
    pub token: String,
    pub expires_at: NaiveDateTime,
}

/// Creates a new password reset token for an account and deletes the previous one if it exists and was not used
pub fn create_password_reset_token(conn: &mut PgConnection, account_id: i32, token: &String, expires_at: &NaiveDateTime) -> diesel::QueryResult<PasswordResetToken> {
    use super::schema::password_reset_tokens::dsl::id;

    let new_password_reset_token = NewPasswordResetToken {
        account_id,
        token: token.to_string(),
        expires_at: expires_at.clone(),
    };

    conn.transaction(|conn| {
        // delete previous token if it exists and was not used
        diesel::delete(password_reset_tokens::dsl::password_reset_tokens.filter(password_reset_tokens::dsl::account_id.eq(account_id)
            .and(password_reset_tokens::dsl::used.eq(false))))
            .execute(conn)?;
        
        insert_into(password_reset_tokens::dsl::password_reset_tokens)
            .values(&new_password_reset_token)
            .execute(conn)?;

        // get newly inserted token
        let token = password_reset_tokens::dsl::password_reset_tokens.select(PasswordResetToken::as_select())
            .order_by(id.desc())
            .first(conn)?;

        Ok(token)
    })
}

pub fn get_password_reset_token(conn: &mut PgConnection, reset_token: &String) -> diesel::QueryResult<PasswordResetToken> {
    use super::schema::password_reset_tokens::dsl::{password_reset_tokens, token};

    password_reset_tokens.select(PasswordResetToken::as_select())
        .filter(token.eq(reset_token))
        .first(conn)
}

pub fn reset_password(conn: &mut PgConnection, reset_token: PasswordResetToken, new_password: &String) -> diesel::QueryResult<()> {
    use super::schema::accounts::dsl::{accounts, password};
    use super::schema::password_reset_tokens::dsl::{password_reset_tokens, used};

    conn.transaction(|conn| {
        // mark token as used
        diesel::update(password_reset_tokens.find(reset_token.id))
            .set(used.eq(true))
            .execute(conn)?;

        diesel::update(accounts.find(reset_token.account_id))
            .set(password.eq(new_password))
            .execute(conn)?;

        Ok(())
    })
}