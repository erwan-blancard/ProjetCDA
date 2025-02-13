// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct CardsElementEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct CardsTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct CosmeticsTypeEnum;
}

diesel::table! {
    account_stats (id) {
        id -> Integer,
        account_id -> Integer,
        first_log -> Datetime,
        last_log -> Datetime,
        games_played -> Bigint,
        games_won -> Bigint,
        wallet -> Bigint,
        experience -> Bigint,
        level -> Integer,
        season_rank -> Integer,
        best_rank -> Integer,
    }
}

diesel::table! {
    accounts (id) {
        id -> Integer,
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
    use super::sql_types::CardsElementEnum;
    use super::sql_types::CardsTypeEnum;

    cards (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 5]
        element -> CardsElementEnum,
        #[sql_name = "type"]
        #[max_length = 6]
        type_ -> CardsTypeEnum,
        stars -> Integer,
        disabled -> Bool,
    }
}

diesel::table! {
    collection_cards (id) {
        id -> Integer,
        account_id -> Integer,
        card_id -> Integer,
    }
}

diesel::table! {
    collection_cosmetics (id) {
        id -> Integer,
        account_id -> Integer,
        cosmetic_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CosmeticsTypeEnum;

    cosmetics (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        price -> Integer,
        #[sql_name = "type"]
        #[max_length = 5]
        type_ -> CosmeticsTypeEnum,
    }
}

diesel::table! {
    friends (id) {
        id -> Integer,
        account1 -> Integer,
        account2 -> Integer,
        status -> Integer,
    }
}

diesel::joinable!(account_stats -> accounts (account_id));
diesel::joinable!(collection_cards -> accounts (account_id));
diesel::joinable!(collection_cards -> cards (card_id));
diesel::joinable!(collection_cosmetics -> accounts (account_id));
diesel::joinable!(collection_cosmetics -> cosmetics (cosmetic_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_stats,
    accounts,
    cards,
    collection_cards,
    collection_cosmetics,
    cosmetics,
    friends,
);
