use utoipa::{openapi::{self, security::{HttpAuthScheme, HttpBuilder, SecurityScheme}}, Modify, OpenApi};


#[derive(OpenApi)]
#[openapi(
    info(
        title = "randomi-go-api",
        description = "The Randomi GO API"
    ),
    paths(
        crate::routes::friends::get_friends_for_account,
        crate::routes::friends::get_friend_requests_for_account,
        crate::routes::friends::send_friend_request,
        crate::routes::friends::change_friend_request_status,
        crate::routes::friends::delete_friendship,
        crate::routes::auth::login,
        crate::routes::auth::register,
        crate::routes::stats::get_my_account_stats,
        crate::routes::stats::get_other_account_stats,
        crate::routes::account::get_my_account,
        crate::routes::account::get_other_account,
        crate::routes::settings::request_password_reset_token,
        crate::routes::settings::reset_password,
        crate::routes::game::create_lobby,
        crate::routes::game::list_lobbies,
        crate::routes::game::lobby_set_ready,
        crate::routes::game::get_current_lobby,
        crate::routes::game::get_lobby_info,
        crate::routes::game::join_lobby,
        crate::routes::game::leave_current_lobby,
        crate::routes::game::get_game_session_info,
        crate::routes::game::get_current_game_session_info,
        crate::routes::game::list_game_sessions,
        crate::routes::sse::event_stream,
        crate::routes::cards::get_cards_collection,
    ),
    components(
        schemas(
            crate::routes::friends::NewFriendRequestJSON,
            crate::routes::friends::FriendRequestResponseJSON,
            crate::database::actions::FriendWithLobbyStatus,
            crate::database::models::Friend,
            crate::database::actions::AccountLogin,
            crate::database::actions::NewAccount,
            crate::database::models::FilteredAccount,
            crate::database::models::AccountStats,
            crate::routes::settings::ResetRequest,
            crate::routes::settings::ResetPassword,
            crate::routes::game::Lobby,
            crate::routes::game::LobbyInfo,
            crate::routes::game::LobbyPageList,
            crate::routes::game::LobbyJoinInfo,
            crate::routes::game::LobbyReadyInfo,
            crate::routes::game::CreateLobbyInfo,
            crate::server::dto::GameSessionInfo,
            crate::routes::cards::CardInfo,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Account", description = "Account profile endpoints"),
        (name = "Settings", description = "Account settings and password reset endpoints"),
        (name = "Stats", description = "Account statistics endpoints"),
        (name = "Friends", description = "Friends management endpoints"),
        (name = "Lobby", description = "Lobby management endpoints"),
        (name = "Game", description = "Game session endpoints"),
        (name = "SSE", description = "Server-Sent Events endpoints for real-time updates"),
        (name = "Cards", description = "Card endpoints"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(
                HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build()
            ),
        );
    }
}
