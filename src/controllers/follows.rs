#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::{follows, products, users};
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct FollowParams {
    pub user_id: i32,
}

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: i32,
    pub pid: String,
    pub username: Option<String>,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub follower_count: i32,
    pub following_count: i32,
    pub is_following: bool,
    pub product_count: u64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct FollowUserResponse {
    pub id: String,
    pub follower_id: i32,
    pub following_id: i32,
    pub created_at: String,
}

// Follow a user
#[debug_handler]
pub async fn follow_user(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<FollowParams>,
) -> Result<Response> {
    let follower = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if follower.id == params.user_id {
        return bad_request("You cannot follow yourself");
    }

    let existing = follows::Entity::find()
        .filter(follows::Column::FollowerId.eq(follower.id))
        .filter(follows::Column::FollowingId.eq(params.user_id))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return bad_request("Already following this user");
    }

    let follow = follows::ActiveModel {
        id: ActiveValue::set(Uuid::new_v4()),
        follower_id: ActiveValue::set(follower.id),
        following_id: ActiveValue::set(params.user_id),
        created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
    };
    let saved_follow = follow.insert(&ctx.db).await?;

    // Increment follower_count on the followed user
    let followed_user = users::Entity::find_by_id(params.user_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let mut followed_active: users::ActiveModel = followed_user.into();
    followed_active.follower_count = ActiveValue::set(Some(
        followed_active.follower_count.unwrap().unwrap_or(0) + 1,
    ));
    followed_active.update(&ctx.db).await?;

    // Increment following_count on the follower
    let mut follower_active: users::ActiveModel = users::Entity::find_by_id(follower.id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?
        .into();
    follower_active.following_count = ActiveValue::set(Some(
        follower_active.following_count.unwrap().unwrap_or(0) + 1,
    ));
    follower_active.update(&ctx.db).await?;

    format::json(FollowUserResponse {
        id: saved_follow.id.to_string(),
        follower_id: saved_follow.follower_id,
        following_id: saved_follow.following_id,
        created_at: saved_follow.created_at.unwrap().to_string(),
    })
}

// Unfollow a user
#[debug_handler]
pub async fn unfollow_user(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<Response> {
    let follower = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let follow = follows::Entity::find()
        .filter(follows::Column::FollowerId.eq(follower.id))
        .filter(follows::Column::FollowingId.eq(user_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    follow.delete(&ctx.db).await?;

    // Decrement follower_count on the unfollowed user
    let followed_user = users::Entity::find_by_id(user_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let mut followed_active: users::ActiveModel = followed_user.into();
    followed_active.follower_count = ActiveValue::set(Some(
        (followed_active.follower_count.unwrap().unwrap_or(1) - 1).max(0),
    ));
    followed_active.update(&ctx.db).await?;

    // Decrement following_count on the follower
    let mut follower_active: users::ActiveModel = users::Entity::find_by_id(follower.id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?
        .into();
    follower_active.following_count = ActiveValue::set(Some(
        (follower_active.following_count.unwrap().unwrap_or(1) - 1).max(0),
    ));
    follower_active.update(&ctx.db).await?;

    format::empty_json()
}

// Check if following
#[debug_handler]
pub async fn check_following(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let is_following = follows::Entity::find()
        .filter(follows::Column::FollowerId.eq(user.id))
        .filter(follows::Column::FollowingId.eq(user_id))
        .one(&ctx.db)
        .await?
        .is_some();

    format::json(serde_json::json!({ "following": is_following }))
}

// Get user profile by username (no auth required)
#[debug_handler]
pub async fn get_user_profile(
    State(ctx): State<AppContext>,
    Path(username): Path<String>,
) -> Result<Response> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(&username))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Get user's products count
    let product_count = products::Entity::find()
        .filter(products::Column::SellerId.eq(user.id))
        .filter(products::Column::Status.eq("active"))
        .count(&ctx.db)
        .await?;

    format::json(UserProfileResponse {
        id: user.id,
        pid: user.pid.to_string(),
        username: user.username.clone(),
        name: user.name,
        email: user.email,
        avatar_url: user.avatar_url,
        location: user.location,
        bio: user.bio,
        follower_count: user.follower_count.unwrap_or(0),
        following_count: user.following_count.unwrap_or(0),
        is_following: false, // Will be updated by separate endpoint if needed
        product_count,
        created_at: user.created_at.to_string(),
    })
}

// Get user's listings
#[debug_handler]
pub async fn get_user_listings(
    State(ctx): State<AppContext>,
    Path(username): Path<String>,
) -> Result<Response> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(&username))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let listings = products::Entity::find()
        .filter(products::Column::SellerId.eq(user.id))
        .filter(products::Column::Status.eq("active"))
        .order_by_desc(products::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    format::json(listings)
}

// Get followers list (simplified)
#[debug_handler]
pub async fn get_followers(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<Response> {
    let followers_list = follows::Entity::find()
        .filter(follows::Column::FollowingId.eq(user_id))
        .all(&ctx.db)
        .await?;

    let follower_ids: Vec<i32> = followers_list.iter().map(|f| f.follower_id).collect();

    let followers_users = if !follower_ids.is_empty() {
        users::Entity::find()
            .filter(users::Column::Id.is_in(follower_ids))
            .all(&ctx.db)
            .await?
    } else {
        vec![]
    };

    let follower_data: Vec<serde_json::Value> = followers_users
        .into_iter()
        .map(|u| {
            serde_json::json!({
                "id": u.id,
                "pid": u.pid,
                "name": u.name,
                "username": u.username,
                "avatar_url": u.avatar_url,
            })
        })
        .collect();

    format::json(follower_data)
}

// Get following list (simplified)
#[debug_handler]
pub async fn get_following(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<Response> {
    let following_list = follows::Entity::find()
        .filter(follows::Column::FollowerId.eq(user_id))
        .all(&ctx.db)
        .await?;

    let following_ids: Vec<i32> = following_list.iter().map(|f| f.following_id).collect();

    let following_users = if !following_ids.is_empty() {
        users::Entity::find()
            .filter(users::Column::Id.is_in(following_ids))
            .all(&ctx.db)
            .await?
    } else {
        vec![]
    };

    let following_data: Vec<serde_json::Value> = following_users
        .into_iter()
        .map(|u| {
            serde_json::json!({
                "id": u.id,
                "pid": u.pid,
                "name": u.name,
                "username": u.username,
                "avatar_url": u.avatar_url,
            })
        })
        .collect();

    format::json(following_data)
}

// Get is_following status for current user
#[debug_handler]
pub async fn get_is_following(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let is_following = follows::Entity::find()
        .filter(follows::Column::FollowerId.eq(current_user.id))
        .filter(follows::Column::FollowingId.eq(user_id))
        .one(&ctx.db)
        .await?
        .is_some();

    format::json(serde_json::json!({ "is_following": is_following }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/follows")
        .add("/follow", post(follow_user))
        .add("/unfollow/{user_id}", delete(unfollow_user))
        .add("/check/{user_id}", get(check_following))
        .add("/profile/{username}", get(get_user_profile))
        .add("/listings/{username}", get(get_user_listings))
        .add("/followers/{user_id}", get(get_followers))
        .add("/following/{user_id}", get(get_following))
        .add("/is-following/{user_id}", get(get_is_following))
}
