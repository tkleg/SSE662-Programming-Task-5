pub fn encrypt(text : &[u8], key: &[u8]) -> Vec<u8> {
    text.iter().enumerate().map(|(i, &c)| {
        c ^ key[i % key.len()]
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_original_string_returned() {
        let text = b"The quick brown fox jumps over the lazy dog.";
        let key = b"0xexcellentParade";
        let encrypted = encrypt(text, key);
        let decrypted = encrypt(&encrypted, key);
        assert_eq!(decrypted, text);
    }

    #[test]
    fn test_encrypt_changes_content() {
        let text = b"Hello, world!";
        let key = b"0xexcellentParade";
        let encrypted = encrypt(text, key);
        assert_ne!(encrypted, text);
    }

    #[test]
    fn test_encrypt_with_short_key() {
        let text = b"abcdefg";
        let key = b"k";
        let encrypted = encrypt(text, key);
        let decrypted = encrypt(&encrypted, key);
        assert_eq!(decrypted, text);
    }

    #[test]
    fn test_encrypt_with_long_text_and_long_key() {
        let text = b";e0r9f0elwbtv-9ugb".repeat(852);
        let key = b"fejifbvfouiec".repeat(56);
        let encrypted = encrypt(&text, &key);
        let decrypted = encrypt(&encrypted, &key);
        assert_eq!(decrypted, text);
    }
}