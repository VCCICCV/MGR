use lettre::Message;
use serde::{ Deserialize, Serialize };

pub mod dto {
    pub mod request;
    pub mod response;
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub from: String,

    pub to: String,

    pub subject: String,

    pub body: String,
}

impl Email {
    pub fn new(from: String, to: String, subject: String, body: String) -> Self {
        Self {
            from,
            to,
            subject,
            body,
        }
    }
}
impl TryFrom<&Email> for Message {
    type Error = anyhow::Error;

    fn try_from(value: &Email) -> Result<Self, Self::Error> {
        Ok(
            Message::builder()
                .from(value.from.parse()?)
                // .reply_to(value.to.parse()?)
                .to(value.to.parse()?)
                .subject(value.subject.clone())
                .body(value.body.clone())?
        )
    }
}
