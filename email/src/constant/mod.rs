use once_cell::sync::Lazy;
use std::time::Duration;
use jsonwebtoken::{ DecodingKey, EncodingKey };

use crate::configure::env::get_env_source;
// 这里的常量需要依赖基础设施层，且仅在基础设施使用

// 环境变量前缀
pub const ENV_PREFIX: &str = "APP";
// 配置
pub static CONFIG: Lazy<crate::configure::AppConfig> = Lazy::new(||
    crate::configure::AppConfig::read(get_env_source(ENV_PREFIX)).unwrap()
);

// Authorization header
pub const AUTHORIZATION: &str = "Authorization";

// 常量
pub const NORMAL_USER: &str = "normal_user";
// 2FA验证码过期时间
pub const EXPIRE_TWO_FACTOR_CODE_SECS: Duration = Duration::from_secs(200);
// Bearer token过期时间
pub const EXPIRE_BEARER_TOKEN_SECS: Duration = Duration::from_secs(600);
// Refresh token过期时间
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(3600);
// 忘记密码验证码过期时间
pub const EXPIRE_FORGET_PASS_CODE_SECS: Duration = Duration::from_secs(120);
// session过期时间约半小时33.333
pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(2000);
// ttl 2FA过期
pub const CHECK_EMAIL_MESSAGE: &str = "Please check you email.";
// 激活用户码过期时间
pub const EXPIRE_ACTIVE_CODE_SECS: Duration = Duration::from_secs(120);
// 验证码长度
pub const CODE_LEN: usize = 6;

// 刷新token加密key
pub static REFRESH_TOKEN_ENCODE_KEY: Lazy<EncodingKey> = Lazy::new(|| {
    let key =
        "-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEArQNJ/bJcY1RP45k4ibWVQig6VyTTLQoHIQjsP230MO5X5jXx
0ro7CFLx4Uy8xmtVgeMvDZYRNh51obpYKnCN5FiZToSEZKhqgUM2oAyAF30gnFQV
V+Ofeyg8G9kNeu3UtR0JGQ+zBj+pLLfzt9E4Bmqv14aGjYmXX7QyAHhUoqVUhDLM
Uw8kJg5tRjrl/52F4dFu8jVtNegkHzL8/DvwbL4Q44X+UQY1vP0AOBr7vkLZnBqX
n/L6DzpMF4AFx2ZFXrT7Vv1z/blaiHDBskiy0q2TXZkGzgoSQKbeS6TgWlN9tokq
p9O/JL7QH59UGsaZnSAhl7eCVJXJmbh6DuBPeQIDAQABAoIBABDbdK6g3cqZ0Ozi
pXUZebEH/WM3RdIqG3a3CVnGy1enKW5XTDgViCHv/vxOP0hv9dVhftiqigPFZP28
i9GaxOB+V3WkiMfL9xCgLWbbwPFI8k/5C1lUZtoXmf+2ns8e9C4vZY8JDg7qTf/6
7ma3HmfVpktwIpUQ1mxvQL31+30bSJ7eObBtVZSiHduDmbJBFKAwHo70TJNWQnUS
zvcnSCK99zab2IBztUPUerqYAUmI01UX8V19fWxBUH9LUxY1Or3Qz30g9zPdg0mN
s+kSCay3gfGQrVtMt2LAl14zV1Cef/4GwcgT6mj2ZUtWCQiEs4S5zZQV4CcSML0x
d1jDri8CgYEA5Uhg0GjwR/M12q0YMS3oh9PJT77vopjLPjqHteXYSgVlXdwqi9+i
/4yWkhP3NpEBdp/wP3ob8dI86auIAR3GriagZCtDbMyGLIaQGeDGdxVmbWmYTq6b
85hlpInozrz6pkZn4/SvHYRz7Ty0U+cw4ASpR4AOKrpwAqGxeGmJUscCgYEAwSxZ
8yYJDOkH5naE0fiVgYB7MVajcUQOxjVH83rETCNjcgncS2cORUL+EkpG2r563Llc
8Mvcpfxft5ciPRbRchfYZudkyB21Bx3XYD7MZD2uedcczlWCf55Ax6BkB1pc2J6g
F1bOVhmTOEpmNISUOFh8iFoIoOkNeEFTQfE7C78CgYBaQwwZ/7XYHV7uH4aOlcwc
7LsrsbcVA6G/dbtHWJy1RzqIoMv/FKNOh0Ifb8y6pos07/Hci23E3UctlmGMTvSQ
SpSJ2p5Ijpujm7XO+jYnWz1fhVp4XzT1yOD6zW/hM3RZUQ5FJ2Zo8GPS2TsEbNdI
2YzqoPbyy0KdRppfFO0CsQKBgG2gBcv/IdO5y0n2qJmgQPglOcReLpLw1JGXWyXJ
3ex3iNAG2IXSDj8wLY+jp2IsBv2MFDpy9t0bxBYkJsIUWPguoBiF6KPIeBt5y+hK
uyOH/aQXd7mAvXY/EPQ5CYtCtL4aBzA3ixFBolKnpijXtGXlkYav0YW/vG0qZV1e
0OtxAoGBAJs5mAx9kpz4/W6xE2UM0I2j5sk1284gPnJDJW9+mB9kHPr80z/7tJFF
TnE+RKm566jeOOJYMWxnxTFc8x3BvqOWNsnrifS9UPdAAhKK7FKTuRsr+Nbz+Yph
+jK33F0E1TnrsD3igZ3LgnxDlnKCAfu9eU6a3uZOx5AMSmRVdRr3
-----END RSA PRIVATE KEY-----
";
    EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
// 刷新token解密key
pub static REFRESH_TOKEN_DECODE_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    let key =
        "-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEArQNJ/bJcY1RP45k4ibWVQig6VyTTLQoHIQjsP230MO5X5jXx0ro7
CFLx4Uy8xmtVgeMvDZYRNh51obpYKnCN5FiZToSEZKhqgUM2oAyAF30gnFQVV+Of
eyg8G9kNeu3UtR0JGQ+zBj+pLLfzt9E4Bmqv14aGjYmXX7QyAHhUoqVUhDLMUw8k
Jg5tRjrl/52F4dFu8jVtNegkHzL8/DvwbL4Q44X+UQY1vP0AOBr7vkLZnBqXn/L6
DzpMF4AFx2ZFXrT7Vv1z/blaiHDBskiy0q2TXZkGzgoSQKbeS6TgWlN9tokqp9O/
JL7QH59UGsaZnSAhl7eCVJXJmbh6DuBPeQIDAQAB
-----END RSA PUBLIC KEY-----
";
    DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
// access token加密key
pub static ACCESS_TOKEN_ENCODE_KEY: Lazy<EncodingKey> = Lazy::new(|| {
    let key =
        "-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAlbmZadu8G83ek/1cIwBCfgxAPQhE45CWhqcnBk57T9b25J6o
VvQrjx55JOhNAZIMNkyTqXtAkoKDTQ6ODT+Oy6r/QefM+BtKYpDH8+60Clk/PHoH
bX0NRatLOZAFD6p1Gd1zFJsjKTnREtJQ65R0mrtaHUAvZ30XPJSBK/f1BMzbpb2j
d05A/+vu8j4wh0U5LDd9dGVMhkLXY61oEmY8Dtm9YcWc8fB6pLL77ODNajdfetld
zRa3NbTyyU3k0HImhHDi1GoRiJpru642OU1nG2D0bn/6FWA5I3lTvytZFbuBkt+F
ZDAMzGiriD6cweaAWYg3wdJciqCy2EIud9daaQIDAQABAoIBAAiDwIy7mUv/fivu
WaIH7sAaVEDkSn6ih1zq/pYmOm8E649+0lX3ls6RzhPiPhbbxmQEINWfk8GBOJRW
Qo/QUH+WnuORmh919dn7H9PofEamYqk+y1NXrDyNDD8eyJrmd7hb1qKSXAKJLjGi
0kVDcQono21MbCW3gG3KiDoG5OE76uZIo6JkuxQe9eamsv1VIBgwlqEoLJbk8fUj
FeSHtXjo5gHj4WWHhVghh9BhLoUy7KxjdDqy1S/i7mcTn5xVz5+XmkG/oe4s1YVP
i5MDo1U3UGaIn+mCqb2etpjuPU9NLzmHmgYfflpRyaB561nce+Op71Z2gggJbrZc
aJEA+e0CgYEAt/v9/WeYokDqBOkcsETw8aY0D5nouJielTOxe7w9lAYS66HS+j7Z
DZlVYSM8YEgNIYtyRhPAmVmQzu5zLu25Llt3JXpB2Y4FXv6P2O9DASEmNg3Ps2jx
h9rY4UK40xyZRHTRZxmB/sTHQxUXVTpup5Bdz29oc6Covc7+u7K1LmcCgYEA0FSt
CjTGn05JsQ0kaRvsvMCOrAiW+LBV07tHwK1rfVYpmSpaZ0Gf6ueWe2dIDEHeSCXu
o+fVeNJyswADQgMLrCnaXBDf6YznW9t5YkPYGE0o+tVjPwo4ewAyzmpBmdsDtIRG
VfKns+yOzBkQ5iUMtfNrusnw/C2GkhIvQlEVDq8CgYEAmHDtcLIOcJmyjOENSSvG
K6LXTfnrLlAeTWdobip5JxWlRaFuarP6Kd9bM8H7O5dXcezVgSTKlwT0C9DclQyZ
O8TDQsU6nGSlJKFtVIRiEySBQZZN+sxAG52Dx5j+BMWUZHkaK/+hixNGOtnV3GMu
3h2q7L3VYMDEL4i73Y9juY8CgYBHN+s/xfLwms7OW/PKiuVPUXR0kyspMP7mNuu8
joxkeIQX/EaTbffBUhI7dX9H0WyvtWfzWFm9yBO15Zc3hdsDq/sd5hblvGmLGWhR
k2fYRuhvd2tKLbBJPDQiJGjws9J8b4ur7hwugfv/AcKpKplByklVvZtIWjfaz6Sg
Qx+zuQKBgHIIldz//3RKfSoaKA+7Clx/oN2JYALu518Xot6TsHvv0uq4RFdCWhBa
rq4zQKK6MJH/jmGAO5D5kuBJWZ28bRqy9p/mOjZ4kFGZ81LKj8roKm530JyKFqGv
NCzTbXqGQoQ/4PooMBPbIPkZWcNssdkSms2kQN318UR4zPhV85XF
-----END RSA PRIVATE KEY-----
";
    EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
// access token解密key
pub static ACCESS_TOKEN_DECODE_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    let key =
        "-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEAlbmZadu8G83ek/1cIwBCfgxAPQhE45CWhqcnBk57T9b25J6oVvQr
jx55JOhNAZIMNkyTqXtAkoKDTQ6ODT+Oy6r/QefM+BtKYpDH8+60Clk/PHoHbX0N
RatLOZAFD6p1Gd1zFJsjKTnREtJQ65R0mrtaHUAvZ30XPJSBK/f1BMzbpb2jd05A
/+vu8j4wh0U5LDd9dGVMhkLXY61oEmY8Dtm9YcWc8fB6pLL77ODNajdfetldzRa3
NbTyyU3k0HImhHDi1GoRiJpru642OU1nG2D0bn/6FWA5I3lTvytZFbuBkt+FZDAM
zGiriD6cweaAWYg3wdJciqCy2EIud9daaQIDAQAB
-----END RSA PUBLIC KEY-----
";
    DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
