#[cfg(test)]
mod pig_latin_interpreter_tests {
    use super::super::{*};
    #[test]
    fn encrypt_message_test() {
        let message = "Hello my name is Jase.";
        let result = encrypt_message(message.to_string());
        assert_eq!(result, "elloHay ymay amenay siay ase.Jay");
    }
    #[test]
    fn decrypt_message_test() {
        let message = "elloHay ymay amenay siay ase.Jay";
        let result = decrypt_message(message.to_string());
        assert_eq!(result, "Hello my name is Jase.");
    }
}