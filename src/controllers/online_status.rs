use crate::models::_entities::users;
use loco_rs::prelude::*;
use sea_orm::{ActiveValue, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OnlineStatusResponse {
    pub is_online: bool,
    pub last_seen: Option<String>,
    pub last_seen_relative: Option<String>,
}

// Update current user's online status (called periodically from frontend)
#[utoipa::path(
    post,
    path = "/api/online-status/heartbeat",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Online status updated"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "online"
)]
#[debug_handler]
pub async fn heartbeat(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    // Find the user by PID from the JWT
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    
    // Update last_seen and is_online
    let mut active_user: users::ActiveModel = user.into();
    active_user.last_seen = ActiveValue::set(Some(chrono::Utc::now().into()));
    active_user.is_online = ActiveValue::set(Some(true));
    active_user.update(&ctx.db).await?;
    
    tracing::info!("Heartbeat received for user: {}", auth.claims.pid);
    
    format::empty_json()
}

// Get user's online status by user ID or username
#[utoipa::path(
    get,
    path = "/api/online-status/user/{identifier}",
    params(
        ("identifier" = String, Path, description = "User ID or username")
    ),
    responses(
        (status = 200, description = "Online status", body = OnlineStatusResponse),
        (status = 404, description = "User not found")
    ),
    tag = "online"
)]
#[debug_handler]
pub async fn get_user_status(
    State(ctx): State<AppContext>,
    Path(identifier): Path<String>,
) -> Result<Response> {
    let user = if let Ok(user_id) = identifier.parse::<i32>() {
        users::Entity::find_by_id(user_id).one(&ctx.db).await?
    } else {
        users::Entity::find()
            .filter(users::Column::Username.eq(&identifier))
            .one(&ctx.db)
            .await?
    };
    
    let user = user.ok_or_else(|| Error::NotFound)?;
    
    // Consider user online if last_seen within last 5 minutes
    let is_online = if let Some(last_seen) = user.last_seen {
        let now = chrono::Utc::now();
        let diff = now.signed_duration_since(last_seen);
        diff.num_minutes() < 5
    } else {
        false
    };
    
    let last_seen_relative = user.last_seen.map(|dt| {
        let now = chrono::Utc::now();
        let diff = now.signed_duration_since(dt);
        
        if diff.num_minutes() < 1 {
            "Just now".to_string()
        } else if diff.num_minutes() < 60 {
            format!("{} minutes ago", diff.num_minutes())
        } else if diff.num_hours() < 24 {
            format!("{} hours ago", diff.num_hours())
        } else {
            format!("{} days ago", diff.num_days())
        }
    });
    
    format::json(OnlineStatusResponse {
        is_online,
        last_seen: user.last_seen.map(|dt| dt.to_string()),
        last_seen_relative,
    })
}

// Batch get online status for multiple users (for chat list)
#[utoipa::path(
    post,
    path = "/api/online-status/batch",
    security(("bearer_auth" = [])),
    request_body = Vec<i32>,
    responses(
        (status = 200, description = "Batch status", body = serde_json::Value)
    ),
    tag = "online"
)]
#[debug_handler]
pub async fn batch_user_status(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(user_ids): Json<Vec<i32>>,
) -> Result<Response> {
    let users_list = users::Entity::find()
        .filter(users::Column::Id.is_in(user_ids))
        .all(&ctx.db)
        .await?;
    
    let status_map: std::collections::HashMap<_, _> = users_list
        .into_iter()
        .map(|u| {
            let is_online = if let Some(last_seen) = u.last_seen {
                let now = chrono::Utc::now();
                let diff = now.signed_duration_since(last_seen);
                diff.num_minutes() < 5
            } else {
                false
            };
            
            let last_seen_relative = if !is_online {
                u.last_seen.map(|dt| {
                    let now = chrono::Utc::now();
                    let diff = now.signed_duration_since(dt);
                    if diff.num_minutes() < 60 {
                        format!("{}m ago", diff.num_minutes())
                    } else if diff.num_hours() < 24 {
                        format!("{}h ago", diff.num_hours())
                    } else {
                        format!("{}d ago", diff.num_days())
                    }
                })
            } else {
                None
            };
            
            (u.id, serde_json::json!({
                "is_online": is_online,
                "last_seen_relative": last_seen_relative,
            }))
        })
        .collect();
    
    format::json(status_map)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/online-status")
        .add("/heartbeat", post(heartbeat))
        .add("/user/{identifier}", get(get_user_status))
        .add("/batch", post(batch_user_status))
}
