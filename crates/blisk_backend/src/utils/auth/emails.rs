use crate::{
    settings::SETTINGS,
    utils::{
        auth::confirmation_token::issue_confirmation_token, emails::send_email,
        errors::ApplicationError, templating::TEMPLATES,
    },
};
use tracing::instrument;

#[instrument(
    name = "Sending a confirmation email",
    skip(redis_con),
    fields(
        recipient_user_id = %uid,
        recipient_name = %recipient_name,
        recipient_email = %recipient_email,
    )
)]
pub async fn send_confirmation_email(
    redis_con: &mut redis::Connection,
    subject: String,
    uid: String,
    recipient_name: String,
    recipient_email: String,
    is_password_change: bool,
) -> Result<(), ApplicationError> {
    let title = subject.clone();

    let issued_token = match issue_confirmation_token(redis_con, uid, is_password_change).await {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    };

    let confirmation_link = {
        if is_password_change {
            format!(
                "{}/auth/change-password?token={}",
                SETTINGS.frontend.url, issued_token,
            )
        } else {
            format!(
                "{}/auth/confirm/?token={}",
                SETTINGS.frontend.url, issued_token,
            )
        }
    };

    let now = chrono::Local::now();
    let ttl = {
        if is_password_change {
            chrono::Duration::hours(1)
        } else {
            chrono::Duration::seconds(SETTINGS.secret.exp)
        }
    };
    let exp = now + ttl;

    let template = TEMPLATES.get_template("confirmation_email.html")?;

    let ctx = minijinja::context! {
        title => &title,
        confirmation_link => &confirmation_link,
        domain => &SETTINGS.frontend.url,
        ttl_minutes => ttl.num_minutes(),
        // Sat, 01 Jun 2024 14:17:00 UTC+7
        expiration_time => &exp.format("%a, %b %d %Y %X UTC%z").to_string()
    };
    let html_text = template.render(ctx).unwrap();

    let text = format!(
        r#"
        Tap the link below to confirm your email address.
        {}
        "#,
        confirmation_link
    );

    send_email(
        None,
        recipient_name,
        recipient_email,
        subject,
        html_text,
        text,
    )
    .await
}
