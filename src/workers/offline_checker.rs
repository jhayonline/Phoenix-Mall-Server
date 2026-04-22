use crate::models::_entities::users;
use chrono::Duration;
use loco_rs::prelude::*;
use sea_orm::{ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};

// Define empty args for the worker
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OfflineCheckerArgs;

pub struct OfflineCheckerWorker {
    pub ctx: AppContext,
}

#[async_trait]
impl BackgroundWorker<OfflineCheckerArgs> for OfflineCheckerWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, _args: OfflineCheckerArgs) -> Result<()> {
        let five_minutes_ago = chrono::Utc::now() - Duration::minutes(5);

        // Mark users as offline if they haven't sent a heartbeat in 5 minutes
        let inactive_users = users::Entity::find()
            .filter(users::Column::IsOnline.eq(true))
            .filter(users::Column::LastSeen.lt(five_minutes_ago))
            .all(&self.ctx.db)
            .await?;

        for user in inactive_users {
            let mut active_user: users::ActiveModel = user.into();
            active_user.is_online = ActiveValue::set(Some(false));
            let _ = active_user.update(&self.ctx.db).await;
        }

        Ok(())
    }
}
