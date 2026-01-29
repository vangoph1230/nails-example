use crate::errors::CustomError;
use axum::{Extension, Json};
use clorinde::{deadpool_postgres::Pool, queries::users::User};

pub async fn loader(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<User>>, CustomError> {
    let client = pool.get().await?;

    let users = clorinde::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

    Ok(Json(users))
}
