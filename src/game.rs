use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

pub fn scramble_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    let mut rng = thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}

pub fn play_game(word: &str) {
    let scrambled = scramble_word(word);
    println!("Unscramble the word: {}", scrambled);

    let mut attempts = 3;

    while attempts > 0 {
        println!("Your guess ({} attempts left):", attempts);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        let guess = guess.trim();

        if guess.eq_ignore_ascii_case(word) {
            println!("✅ Correct! You win!");
            return;
        } else {
            println!("❌ Incorrect.");
            attempts -= 1;
        }
    }
    println!("💡 Out of attempts. The correct word was: {}", word);
}
