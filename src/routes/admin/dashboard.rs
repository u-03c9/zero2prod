use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

pub async fn admin_dashboard(
    db_pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
        get_username(user_id, &db_pool).await.map_err(e500)?
    } else {
        return Ok(see_other("/login"));
    };
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Admin dashboard</title>
            </head>
            <body>
                <p>Welcome {}!</p>
                <p>Available actions:</p>
                <ol>
                    <li><a href="/admin/password">Change password</a><li>
                    <li>
                        <a href="javascript:document.logoutForm.submit()">Logout</a>
                        <form name="logoutForm" action="/admin/logout" method="post" hidden>
                            <input hidden type="submit" value="Logout">
                        </form>
                    </li>
                </ol>
            </body>
            </html>
            "#,
            username
        )))
}

#[tracing::instrument(name = "Get username", skip(db_pool))]
pub async fn get_username(user_id: Uuid, db_pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(db_pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;
    Ok(row.username)
}
