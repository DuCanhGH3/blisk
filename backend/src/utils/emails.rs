use crate::{
    settings::SETTINGS,
    utils::{auth::confirmation_token::issue_confirmation_token, templating::TEMPLATES},
};
use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use tracing::{event, instrument, Level};

#[instrument(
    name = "Sending an email",
    skip(
        recipient_name,
        recipient_email,
        subject,
        html_content,
        text_content
    ),
    fields(
        recipient_name = %recipient_name,
        recipient_email = %recipient_email,
    )
)]
pub async fn send_email(
    sender_email: Option<String>,
    recipient_name: String,
    recipient_email: String,
    subject: impl Into<String>,
    html_content: impl Into<String>,
    text_content: impl Into<String>,
) -> Result<(), String> {
    let email = match Message::builder()
        .from(
            format!(
                "{} <{}>",
                "blisk",
                if sender_email.is_some() {
                    sender_email.unwrap()
                } else {
                    SETTINGS.email.host.name.clone()
                }
            )
            .parse()
            .unwrap(),
        )
        .to(format!("{} <{}>", recipient_name, recipient_email)
            .parse()
            .unwrap())
        .subject(subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                        .body(text_content.into()),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(html_content.into()),
                ),
        ) {
        Ok(message) => message,
        Err(err) => {
            event!(
                Level::ERROR,
                "(Lettre) failed to construct message: {}",
                err
            );
            return Err(format!("{}", err));
        }
    };

    let creds = Credentials::new(
        SETTINGS.email.host.name.clone(),
        SETTINGS.email.host.password.clone(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&SETTINGS.email.host.name)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(email).await {
        Ok(_) => {
            event!(Level::INFO, "email sent!");
            Ok(())
        }
        Err(e) => {
            event!(Level::ERROR, "(Lettre) failed to send email: {:#?}", e);
            Err(format!("Failed to send email: {:#?}", e))
        }
    }
}

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
) -> Result<(), String> {
    let title = subject.clone();

    let issued_token = match issue_confirmation_token(redis_con, uid, is_password_change).await {
        Ok(t) => t,
        Err(e) => {
            event!(Level::ERROR, e);
            return Err(e);
        }
    };

    let web_address = {
        if SETTINGS.application.debug {
            format!(
                "{}:{}",
                SETTINGS.application.base, SETTINGS.application.port,
            )
        } else {
            SETTINGS.application.base.clone()
        }
    };

    let confirmation_link = {
        if is_password_change {
            format!(
                "{}/users/change-password?token={}",
                web_address, issued_token,
            )
        } else {
            format!("{}/users/confirm/?token={}", web_address, issued_token,)
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

    let template = match TEMPLATES.get_template("confirmation_email.html") {
        Ok(template) => template,
        Err(err) => {
            event!(
                Level::ERROR,
                "(MiniJinja) failed to retrieve template: {}",
                err
            );
            return Err(format!("{}", err));
        }
    };

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
