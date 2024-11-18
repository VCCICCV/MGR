// 消息类型

pub enum MessageKind {
    // 激活码
    ActiveCode,
    // 登录验证码
    LoginCode,
    // 忘记密码验证码
    ForgetPasswordCode,
}
// 消息状态

pub enum MessageStatus {
    // 待发送
    Pending,
    // 发送中
    Sending,
    // 发送成功
    Success,
    // 发送失败
    Failed,
}
