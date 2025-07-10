use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;


#[derive(Debug, Clone)]
pub struct Mailer {
    mailer: SmtpTransport,
    from: String,
}

impl Mailer {
    pub fn create() -> Self {
        let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER not set");
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");
        let smtp_from = env::var("SMTP_FROM").expect("SMTP_FROM not set");

        let mailer = SmtpTransport::relay(&smtp_server)
            .unwrap()
            .credentials(Credentials::new(smtp_username, smtp_password))
            .build();

        Self {
            mailer,
            from: smtp_from,
        }
    }

    pub fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let email = Message::builder()
            .from(self.from.parse().map_err(|e| format!("Invalid FROM address: {e}"))?)
            .to(to.parse().map_err(|e| format!("Invalid TO address: {e}"))?)
            .subject(subject)
            .body(body.to_string())
            .map_err(|e| format!("Failed to build email: {e}"))?;
    
        self.mailer.send(&email)
            .map_err(|e| format!("Failed to send email: {e}"))?;
    
        Ok(())
    }

    pub fn send_password_reset_email(&self, to: &str, token: &str) -> Result<(), String> {
        let website_url = env::var("WEBSITE_URL").map_err(|_| "WEBSITE_URL not set")?;
        let subject = "Password Reset Request";
        let body = format!(
            "Hello,\n\nYou requested a password reset. Click the link below to reset your password:\n{website_url}/reset-password?token={token}\n\nIf you did not request this, please ignore this email.\n\nBest regards,\nRandomi GO Team"
        );
        self.send_email(to, &subject, &body)
    }
}
