use kernel::{Error, Result};

use super::channels::{email, whatsapp};
use super::config::NotificationsConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistrationChannel {
    Email,
    Whatsapp,
}

pub async fn send_registration_otp(
    config: &NotificationsConfig,
    channel: RegistrationChannel,
    destination: &str,
    language: &str,
    code: &str,
) -> Result<()> {
    let template = &super::copy::REGISTRATION_OTP;
    let subject = super::render::pick(language, template.title_en, template.title_fr);
    let body = super::render::pick(language, template.body_en, template.body_fr).replace("{code}", code);
    let send = async {
        match channel {
            RegistrationChannel::Email => {
                if config.shipmail_api_key.is_empty() {
                    return Err(Error::internal("OTP email provider is not configured"));
                }
                email::send(config, destination, subject, &body, None, &[])
                    .await.map_err(|_| Error::internal("OTP email delivery failed"))
            }
            RegistrationChannel::Whatsapp => whatsapp::send_message(config, destination, &body)
                .await.map_err(|_| Error::internal("OTP WhatsApp delivery failed")),
        }
    };
    tokio::time::timeout(std::time::Duration::from_secs(8), send)
        .await.map_err(|_| Error::internal("OTP delivery timed out"))?
}
