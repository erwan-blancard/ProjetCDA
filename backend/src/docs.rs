use utoipa::{openapi::{self, security::{HttpAuthScheme, HttpBuilder, SecurityScheme}}, Modify, OpenApi};


#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::friends::get_friends_for_account,
        crate::routes::friends::get_friend_requests_for_account,
        crate::routes::friends::send_friend_request,
        crate::routes::friends::change_friend_request_status,
        crate::routes::friends::delete_friendship,
    ),
    components(
        schemas(
            crate::routes::friends::NewFriendRequestJSON,
            crate::routes::friends::FriendRequestResponseJSON,
            crate::database::actions::FriendWithLobbyStatus,
            crate::database::models::Friend,
        )
    ),
    tags(
        (name = "Friends", description = "Friends management endpoints")
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
