use rand::{ distributions::Alphanumeric, Rng };
// 生成随机字符串
pub fn generate_random_string(len: usize) -> String {
    rand::rng().sample_iter(&Alphanumeric).take(len).map(char::from).collect()
}
// 生成随机字符串并加上前缀
pub fn generate_random_string_with_prefix(prefix: &str) -> String {
    format!("{prefix}_{}", generate_random_string(10))
}

#[cfg(test)]
mod tests {
    use fake::{ Fake, Faker };

    use super::*;

    #[test]
    fn test_generate_random_string_with_prefix() {
        let prefix: String = Faker.fake();
        let result = generate_random_string_with_prefix(&prefix);
        println!("result: {result}");
        assert!(result.starts_with(&prefix));
    }

    #[test]
    fn test_generate_random_string() {
        let len = 4;
        let name = generate_random_string(len);
        println!("name: {name}");
        assert_eq!(name.len(), len);
    }
    #[test]
    fn test_generate_random_code() {
        let len = 6;
        let code = generate_random_string(len);
        println!("code: {code}");
        assert_eq!(code.len(), 6);
    }
}
