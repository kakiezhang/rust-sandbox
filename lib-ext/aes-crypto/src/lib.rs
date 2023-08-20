pub fn encrypt(x: usize) -> usize {
    x * 10
}

pub fn decrypt(x: usize) -> usize {
    x / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let result = encrypt(2);
        assert_eq!(result, 20);
    }

    #[test]
    fn test_decrypt() {
        let result = decrypt(29);
        assert_eq!(result, 2);
    }
}
