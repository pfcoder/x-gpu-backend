use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::Result,
    model::{CreateUserData, UpdateUserData, User},
};

impl User {
    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<User> {
        let sql = format!("SELECT * FROM {} WHERE id = $1 LIMIT 1", User::TABLE);
        Ok(sqlx::query_as(&sql).bind(id).fetch_one(pool).await?)
    }

    pub async fn create(data: CreateUserData, pool: &PgPool) -> Result<User> {
        // ON CONFLICT id DO UPDATE SET access_token = $2, refresh_token = $3, expires_at = $4, updated_at = $6
        let sql = format!(
            "
            INSERT INTO {} (id, access_token, refresh_token, expires_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            ",
            User::TABLE
        );
        tracing::info!("create user sql: {}", sql);
        Ok(sqlx::query_as(&sql)
            .bind(data.id)
            .bind(data.access_token)
            .bind(data.refresh_token)
            .bind(data.expires_at)
            .bind(data.created_at)
            .bind(data.updated_at)
            .fetch_one(pool)
            .await?)
    }

    pub async fn update(id: Uuid, data: UpdateUserData, pool: &PgPool) -> Result<User> {
        let sql = format!(
            "
            UPDATE {}
            SET access_token = $2, refresh_token = $3, expires_at = $4, updated_at = $5
            WHERE id = $1
            RETURNING *
            ",
            User::TABLE
        );
        Ok(sqlx::query_as(&sql)
            .bind(id)
            .bind(data.access_token)
            .bind(data.refresh_token)
            .bind(data.expires_at)
            .bind(data.updated_at)
            .fetch_one(pool)
            .await?)
    }
}
