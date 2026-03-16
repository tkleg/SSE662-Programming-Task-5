pub fn encrypt(text : &[u8], key: &[u8]) -> Vec<u8> {
    let cipher : Vec<u8> = key.to_vec();
    let encrypted_contents : Vec<u8> = text.iter().enumerate().map(|(i, &c)| {
        let cipher_char = cipher[i % cipher.len()];
        let encrypted_char = (c as u8) ^ (cipher_char as u8);
        encrypted_char as u8
    }).collect();
    encrypted_contents
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
}