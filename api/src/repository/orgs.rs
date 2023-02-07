use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

pub async fn has_org(pool: &PgPool, account_id: Uuid) -> Result<bool, AppError> {
    let possible_org = sqlx::query!(
        "select org_id from orgs.members where member_id = $1",
        account_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    Ok(possible_org.is_some())
}