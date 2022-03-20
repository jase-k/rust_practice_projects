use std::io;

#[cfg(test)]
#[path = "pig_latin_interpreter_tests.rs"]
mod pig_latin_interpreter_tests;

pub fn interpret() -> () {
    println!(
        "Do want your result in English or in Pig Latin? \n Type PIG or ENG:"
    );

    let mut choice = String::new();
    let mut og_message = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to Read Line");

    let choice = choice.trim();

    println!(
        "Type your message:"
    );

    io::stdin()
        .read_line(&mut og_message)
        .expect("Failed to Read Message");

    let message;
    if choice == "PIG" {
        message = encrypt_message(og_message);
    } 
    else if choice == "ENG" {
        message = decrypt_message(og_message);
    } else {
        println!("Invalid option. Please type PIG or ENG");
        return ()
    }
    println!("Your Message:  {}", message);
}

fn encrypt_message(message: String) -> String {
    let string_vector: Vec<&str> = message.split_whitespace().collect();
    let v: Vec<String> = string_vector.into_iter().map(|x| encrypt_word(&x)).collect();
    v.join(" ")
}

fn encrypt_word(s: &str) -> String {
    let mut word: String = String::new();
    word.push_str(&s[1..]);
    word.push_str(&s[0..1]);
    word.push_str("ay");
    word
}

fn decrypt_word(s: &str) -> String {
    let end: usize = s.len() - 2;
    let mut word: String = String::new();
    word.push_str(&s[end-1..end]);
    word.push_str(&s[0..end-1]);
    word
}

fn decrypt_message(message: String) -> String {
    let string_vector: Vec<&str> = message.split_whitespace().collect();
    let v: Vec<String> = string_vector.into_iter().map(|x| decrypt_word(&x)).collect();
    v.join(" ")
}