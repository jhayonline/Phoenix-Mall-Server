use crate::models::_entities::users;
use loco_rs::prelude::*;

pub async fn require_admin(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    req: Request,
    next: Next,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if user.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    Ok(next.run(req).await)
}
