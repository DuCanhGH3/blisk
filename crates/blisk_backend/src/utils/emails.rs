use crate::settings::SETTINGS;
use crate::utils::errors::AppError;
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
) -> Result<(), AppError> {
    let email = Message::builder()
        .from(
            format!(
                "blisk <{}>",
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
        )?;

    let creds = Credentials::new(
        SETTINGS.email.host.name.clone(),
        SETTINGS.email.host.password.clone(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&SETTINGS.email.host.name)
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(email).await?;
    event!(Level::INFO, "email sent!");
    Ok(())
}
