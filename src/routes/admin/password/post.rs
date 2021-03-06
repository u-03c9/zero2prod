use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;

use crate::authentication::{validate_credentials, AuthError, Credentials};
use crate::routes::admin::dashboard::get_username;
use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    session: TypedSession,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = session.get_user_id().map_err(e500)?;
    if user_id.is_none() {
        return Ok(see_other("/login"));
    }
    let user_id = user_id.unwrap();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();
        return Ok(see_other("/admin/password"));
    }
    if let Err(error_msg) = validate_password_strength(&form.new_password) {
        FlashMessage::error(error_msg).send();
        return Ok(see_other("/admin/password"));
    }

    let username = get_username(user_id, &db_pool).await.map_err(e500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };

    if let Err(e) = validate_credentials(credentials, &db_pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/admin/password"))
            }
            AuthError::UnexpectedError(_) => Err(e500(e).into()),
        };
    }
    crate::authentication::change_password(user_id, form.0.new_password, &db_pool)
        .await
        .map_err(e500)?;
    FlashMessage::error("Your password has been changed.").send();
    Ok(see_other("/admin/password"))
}

fn validate_password_strength(password: &Secret<String>) -> Result<(), &str> {
    // TODO: currently only checking for the passwords's length
    // I should use a dedicated crate like `zxcvbn`
    // https://docs.rs/zxcvbn/2.1.2/zxcvbn/
    match password.expose_secret().graphemes(true).count() {
        i if i <= 12 => Err("The new password is too short"),
        i if i >= 128 => Err("The new password is too long"),
        _ => Ok(()),
    }
}
