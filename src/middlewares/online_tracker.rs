use crate::models::_entities::users;
use loco_rs::prelude::*;
use sea_orm::{ActiveValue, EntityTrait};

pub async fn track_online_activity(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    req: Request,
    next: Next,
) -> Result<Response> {
    // Update user's last seen timestamp
    if let Ok(user) = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        let mut active_user: users::ActiveModel = user.into();
        active_user.last_seen = ActiveValue::set(Some(chrono::Utc::now().into()));
        active_user.is_online = ActiveValue::set(Some(true));
        let _ = active_user.update(&ctx.db).await;
    }

    Ok(next.run(req).await)
}
