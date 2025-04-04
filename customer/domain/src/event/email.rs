use lettre::Message;
use serde::{ Deserialize, Serialize };
pub enum Event{
    Email(Email),
}
// 这里定义从系统产生的对象，但是是请求其他系统的DTO
// 邮件数据传输对象
#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub from: String,
    pub reply_to: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn new(from: String, reply_to: String, to: String, subject: String, body: String) -> Self {
        Self {
            from,
            reply_to,
            to,
            subject,
            body,
        }
    }
}
// 将 Email 转换为 Message
impl TryFrom<&Email> for Message {
    type Error = anyhow::Error;
    fn try_from(value: &Email) -> Result<Self, Self::Error> {
        Ok(
            Message::builder()
                .from(value.from.parse()?)
                .reply_to(value.reply_to.parse()?)
                .to(value.to.parse()?)
                .subject(value.subject.clone())
                .body(value.body.clone())?
        )
    }
}