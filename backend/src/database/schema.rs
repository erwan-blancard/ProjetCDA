// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "card_element"))]
    pub struct CardElement;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "card_type"))]
    pub struct CardType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "cosmetic_type"))]
    pub struct CosmeticType;
}

diesel::table! {
    account_stats (id) {
        id -> Int4,
        account_id -> Int4,
        first_log -> Timestamp,
        last_log -> Timestamp,
        games_played -> Int8,
        games_won -> Int8,
        wallet -> Int8,
        experience -> Int8,
        level -> Int4,
        season_rank -> Int4,
        best_rank -> Int4,
    }
}

diesel::table! {
    accounts (id) {
        id -> Int4,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        premium -> Bool,
        suspended -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CardElement;
    use super::sql_types::CardType;

    cards (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        element -> CardElement,
        #[sql_name = "type"]
        type_ -> CardType,
        stars -> Int4,
        disabled -> Bool,
    }
}

diesel::table! {
    collection_cards (id) {
        id -> Int4,
        account_id -> Int4,
        card_id -> Int4,
    }
}

diesel::table! {
    collection_cosmetics (id) {
        id -> Int4,
        account_id -> Int4,
        cosmetic_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CosmeticType;

    cosmetics (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        price -> Int4,
        #[sql_name = "type"]
        type_ -> CosmeticType,
    }
}

diesel::table! {
    friends (id) {
        id -> Int4,
        account1 -> Int4,
        account2 -> Int4,
        status -> Int4,
    }
}

diesel::table! {
    password_reset_tokens (id) {
        id -> Int4,
        account_id -> Int4,
        #[max_length = 36]
        token -> VarChar,
        expires_at -> Timestamp,
        used -> Bool,
    }
}

diesel::joinable!(account_stats -> accounts (account_id));
diesel::joinable!(collection_cards -> accounts (account_id));
diesel::joinable!(collection_cards -> cards (card_id));
diesel::joinable!(collection_cosmetics -> accounts (account_id));
diesel::joinable!(collection_cosmetics -> cosmetics (cosmetic_id));
diesel::joinable!(password_reset_tokens -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_stats,
    accounts,
    cards,
    collection_cards,
    collection_cosmetics,
    cosmetics,
    friends,
    password_reset_tokens,
);
