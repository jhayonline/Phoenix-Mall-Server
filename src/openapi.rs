use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

pub struct ApiDoc;

impl ApiDoc {
    pub fn openapi() -> utoipa::openapi::OpenApi {
        #[derive(OpenApi)]
        #[openapi(
            info(
                title = "Phoenix Marketplace API",
                description = "API documentation for Phoenix Online Marketplace",
                version = "1.0.0",
                contact(
                    name = "API Support",
                    email = "support@phoenixmarketplace.com"
                ),
                license(
                    name = "MIT",
                    url = "https://opensource.org/licenses/MIT"
                )
            ),
            servers(
                (url = "/api", description = "API Server")
            ),
            components(
                schemas(
                    // Auth
                    crate::models::users::RegisterParams,
                    crate::models::users::LoginParams,
                    crate::views::auth::LoginResponse,
                    crate::views::auth::CurrentResponse,
                    crate::views::profile::ProfileResponse,
                    crate::views::profile::UserStatsResponse,
                    crate::controllers::auth::ForgotParams,
                    crate::views::user_response::UserResponse,
                    crate::controllers::auth::ResetParams,
                    crate::controllers::auth::MagicLinkParams,
                    crate::controllers::auth::ResendVerificationParams,
                    crate::controllers::admin::StatsResponse,
                    crate::controllers::chat::MessageResponse,
                    crate::controllers::chat::ConversationResponse,
                    crate::controllers::follows::UserProfileResponse,
                    crate::controllers::follows::FollowUserResponse,
                    crate::controllers::wishlist::WishlistResponse,
                    
                    // Users
                    crate::models::users::UpdateProfileParams,
                    crate::controllers::admin::UpdateUserStatusParams,
                    crate::controllers::admin::UpdateUserRoleParams,
                    
                    // Products
                    crate::models::products::CreateProductParams,
                    crate::models::products::UpdateProductParams,
                    crate::models::products::ProductQueryParams,
                    crate::models::products::PaginatedProductsResponse,
                    
                    // Categories
                    crate::controllers::categories::CreateCategoryParams,
                    crate::controllers::categories::UpdateCategoryParams,
                    
                    // Wishlist
                    crate::controllers::wishlist::AddToWishlistParams,
                    
                    // Chat
                    crate::controllers::chat::SendMessageParams,
                    
                    // Follows
                    crate::controllers::follows::FollowParams,

                    // Views
                    crate::views::product_response::ProductResponse,
                )
            ),
            tags(
                (name = "auth", description = "Authentication endpoints"),
                (name = "users", description = "User management"),
                (name = "products", description = "Product management"),
                (name = "categories", description = "Category management"),
                (name = "favorites", description = "Favorites and wishlist"),
                (name = "chat", description = "Messaging system"),
                (name = "follows", description = "Social following"),
                (name = "profile", description = "User profile"),
                (name = "admin", description = "Admin operations"),
                (name = "images", description = "Image upload and management"),
            ),
            modifiers(&SecurityAddon)
        )]
        struct ApiDoc;
        
        ApiDoc::openapi()
    }
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            );
        }
    }
}
