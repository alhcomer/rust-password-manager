#[cfg(test)]
mod core_tests {
    use crate::core::generate_password;

    #[test]
    fn test_generate_password_length() {
        let length = 10;
        let password = generate_password(length);
        assert_eq!(password.len(), length as usize);
    }

    #[test]
    fn test_generate_password_characters() {
        let length = 8;
        let password = generate_password(length);
        let charset: Vec<u8> = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(*&^%$#@!~".to_vec();
        let password_chars: Vec<char> = password.chars().collect();

        for c in password_chars {
            assert!(charset.contains(&(c as u8)));
        }
    }
}